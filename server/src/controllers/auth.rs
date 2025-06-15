use actix_web::web::{self, Json};
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
use base64::{Engine, engine::general_purpose};
use diesel::{QueryDsl, SelectableHelper, result::Error::NotFound};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    app::AppState,
    auth::{AuthenticatedUser, create_token},
    models::{
        refresh_tokens::{self, RefreshToken},
        user::User,
    },
    util::errors::{AppError, AppResult},
};

use super::helper::OkResponse;

#[derive(Deserialize, Serialize, ToSchema, Debug, Clone)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

pub enum ValidateResult<'a> {
    Error(&'a str),
    Found(&'a str),
}

const ACCESS_TOKEN_EXPIRATION: i64 = 10; // 10 mins

const REFRESH_TOKEN_EXPIRATION: i64 = 10080; // 1 week

const AUTH: &str = "authentication";

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetAccessTokenResponse {
    pub access_token: String,
}

#[utoipa::path(
    tag = "development",
    post,
    path = "/api/dev/auth/access-token",
    description = "",
    responses(
        (status = 200, description = "Successful response", body = GetAccessTokenResponse)
    )
)]
pub async fn get_access_token(
    state: web::Data<AppState>,
) -> AppResult<Json<GetAccessTokenResponse>> {
    let token = create_token(&Uuid::new_v4(), &state.config.jwt_config.access_secret, 10);

    Ok(Json(GetAccessTokenResponse {
        access_token: token,
    }))
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LoginResponse {
    access_token: String,
    refresh_token: String,
}

#[utoipa::path(
    tag = AUTH,
    post,
    path = "/api/auth/login",
    responses(
        (status = 200, description = "Login was successful", body = LoginResponse)
    ),
)]
pub async fn login(
    credentials: web::Json<LoginCredentials>,
    state: web::Data<AppState>,
) -> AppResult<Json<LoginResponse>> {
    let mut conn = state.db_connection().await?;

    let result = User::find_by_email(&mut conn, &credentials.email).await;

    match result {
        Ok(user) => {
            let user_password_salt = user.password_salt;
            let user_password_hash = user.password_hash;

            if user_password_hash.is_none() || user_password_salt.is_none() {
                return Err(AppError::Unauthorized);
            }

            let salt = match SaltString::from_b64(
                &general_purpose::STANDARD_NO_PAD.encode(user_password_salt.unwrap()),
            ) {
                Ok(res) => res,
                _ => return Err(AppError::InternalError),
            };

            let argon2 = Argon2::default();

            let password_hash = match argon2.hash_password(credentials.password.as_bytes(), &salt) {
                Ok(hash) => hash.to_string(),
                _ => return Err(AppError::InternalError),
            };

            if password_hash == user_password_hash.unwrap() {
                let access_token_secret = state.config.jwt_config.access_secret.clone();
                let refresh_token_secret = state.config.jwt_config.refresh_secret.clone();

                let access_token =
                    create_token(&user.id, &access_token_secret, ACCESS_TOKEN_EXPIRATION);
                let refresh_token =
                    create_token(&user.id, &refresh_token_secret, REFRESH_TOKEN_EXPIRATION);

                return Ok(Json(LoginResponse {
                    access_token,
                    refresh_token,
                }));
            }

            Err(AppError::Unauthorized)
        }
        Err(NotFound) => Err(AppError::Unauthorized),
        Err(_) => Err(AppError::InternalError),
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LogoutBody {
    pub refresh_token: String,
}

#[utoipa::path(
    tag=AUTH,
    post,
    path="/"
)]
pub async fn logout(
    authenticated: AuthenticatedUser,
    body: web::Json<LogoutBody>,
    state: web::Data<AppState>,
) -> AppResult<OkResponse> {
    use crate::schema::refresh_tokens;

    let mut conn = state.db_connection().await?;
    let user_id = authenticated.0;
    let refresh_token_hash = hex::encode(Sha256::digest(body.refresh_token.as_bytes()));

    let refresh_token: RefreshToken = refresh_tokens::table
        .find(refresh_token_hash)
        .select(RefreshToken::as_select())
        .first(&mut conn)
        .await
        .map_err(|_| AppError::Unauthorized)?;

    if refresh_token.user_id != Some(user_id) {
        return Err(AppError::Unauthorized);
    }

    match refresh_token.revoke(&mut conn).await {
        Ok(_) => Ok(OkResponse::default()),
        Err(_) => Err(AppError::InternalError),
    }
}
