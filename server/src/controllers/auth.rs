use actix_web::web::{self, Json};
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
use base64::{Engine, engine::general_purpose};
use diesel::result::Error::NotFound;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    app::AppState,
    auth::create_access_token,
    models::user::User,
    util::errors::{AppError, AppResult},
};

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

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LoginResponse {
    access_token: String,
    refresh_token: String,
}

#[utoipa::path(
    tag = "test",
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
                    create_access_token(user.id, &access_token_secret, ACCESS_TOKEN_EXPIRATION);
                let refresh_token =
                    create_access_token(user.id, &refresh_token_secret, REFRESH_TOKEN_EXPIRATION);

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
