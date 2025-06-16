use actix_web::web::{self, Json};
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
use base64::{Engine, engine::general_purpose};
use diesel::result::Error::NotFound;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    app::AppState,
    auth::{AuthenticatedUser, create_token},
    models::{refresh_tokens::RefreshToken, user::User},
    util::errors::{AppError, AppResult},
    views::EncodableUser,
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
    ),
)]
pub async fn get_access_token(
    state: web::Data<AppState>,
) -> AppResult<Json<GetAccessTokenResponse>> {
    let token = create_token(&Uuid::new_v4(), &state.config.jwt_config.access_secret, 10);

    Ok(Json(GetAccessTokenResponse {
        access_token: token,
    }))
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct GetMeResponse {
    pub user: EncodableUser,
}

#[utoipa::path(
    tag = AUTH,
    get,
    path = "/api/v1/auth/me",
    responses(
        (status = 200, description = "Successful response", body = GetMeResponse),
        (status = 404, description = "User not found")
    ),
)]
pub async fn get_me(
    authenticated: AuthenticatedUser,
    state: web::Data<AppState>,
) -> AppResult<Json<GetMeResponse>> {
    let user_id = authenticated.0;

    let mut conn = state.db_connection().await?;

    match User::find(&mut conn, &user_id).await {
        Ok(user) => Ok(Json(GetMeResponse {
            user: EncodableUser::from(user),
        })),
        Err(_) => Err(AppError::NotFound),
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[utoipa::path(
    tag = AUTH,
    post,
    path = "/api/v1/auth/login",
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

                let exp = state.config.jwt_config.refresh_token_expiration;

                return match RefreshToken::create(&mut conn, &refresh_token, &user.id, exp).await {
                    Ok(_) => Ok(Json(LoginResponse {
                        access_token,
                        refresh_token,
                    })),
                    Err(_) => Err(AppError::InternalError),
                };
            }

            Err(AppError::Unauthorized)
        }
        Err(NotFound) => Err(AppError::Unauthorized),
        Err(_) => Err(AppError::InternalError),
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RefreshTokenBody {
    pub refresh_token: String,
}

#[utoipa::path(
    tag=AUTH,
    post,
    path="/api/v1/auth/logout",
    responses(
        (status = 200, description = "Logout was successful", body = OkResponse),
        (status = 401, description = "Invalid or missing token"),
        (status = 500, description = "Internal server error"),
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn logout(
    authenticated: AuthenticatedUser,
    body: web::Json<RefreshTokenBody>,
    state: web::Data<AppState>,
) -> AppResult<OkResponse> {
    let mut conn = state.db_connection().await?;
    let user_id = authenticated.0;

    let refresh_token = RefreshToken::find(&mut conn, &body.refresh_token)
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[utoipa::path(
    tag=AUTH,
    post,
    path="/api/v1/auth/refresh",
    responses(
        (status = 200, description = "Refresh was succesful", body = RefreshResponse)
    )
)]
pub async fn refresh(
    body: web::Json<RefreshTokenBody>,
    state: web::Data<AppState>,
) -> AppResult<Json<RefreshResponse>> {
    let mut conn = state.db_connection().await?;

    let refresh_token = RefreshToken::find(&mut conn, &body.refresh_token)
        .await
        .map_err(|_| AppError::Unauthorized)?;

    if refresh_token.revoked {
        return Err(AppError::Unauthorized);
    }

    if let Some(user_id) = refresh_token.user_id {
        let secret = &state.config.jwt_config.refresh_secret;
        let access_expiration_time = state.config.jwt_config.access_token_expiration;
        let refresh_expiration_time = state.config.jwt_config.refresh_token_expiration;

        match refresh_token
            .issue_new(&mut conn, secret, refresh_expiration_time)
            .await
        {
            Ok(refresh_token) => {
                let access_token = create_token(&user_id, secret, access_expiration_time);

                Ok(Json(RefreshResponse {
                    access_token,
                    refresh_token,
                }))
            }
            Err(_) => Err(AppError::InternalError),
        }
    } else {
        Err(AppError::Unauthorized)
    }
}
