use actix_identity::Identity;
use actix_web::{HttpMessage, HttpRequest, web};
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
    models::user::{LoggedUser, User},
    util::errors::{AppError, AppResult},
};

#[derive(Deserialize, Serialize, ToSchema, Debug, Clone)]
pub struct LoginCredentials {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

pub enum ValidateResult<'a> {
    Error(&'a str),
    Found(&'a str),
}

const GENERIC_BAD_REQUEST: &str = "Check username/email and password.";

#[utoipa::path(
    tag = "test",
    post,
    path = "/api/auth/login",
    responses(
        (status = 200, description = "Login was successful", body = str)
    ),
)]
pub async fn login(
    credentials: web::Json<LoginCredentials>,
    req: HttpRequest,
    state: web::Data<AppState>,
) -> AppResult<&'static str> {
    let validate = |user: User| -> Result<(), AppError> {
        let salt = match SaltString::from_b64(
            &general_purpose::STANDARD_NO_PAD.encode(&user.password_salt),
        ) {
            Ok(res) => res,
            _ => return Err(AppError::InternalError),
        };

        let argon2 = Argon2::default();

        let password_hash = match argon2.hash_password(credentials.password.as_bytes(), &salt) {
            Ok(hash) => hash.to_string(),
            _ => return Err(AppError::InternalError),
        };

        if password_hash == user.password_hash {
            let logged_user = LoggedUser::from(user.clone());

            let logged_user_string = serde_json::to_string(&logged_user).unwrap();

            let _ = Identity::login(&req.extensions(), logged_user_string);

            return Ok(());
        };

        Err(AppError::BadRequest {
            field: GENERIC_BAD_REQUEST.to_string(),
        })
    };

    if let Some(username) = &credentials.username {
        let mut conn = state.db_connection().await?;

        let result = User::find_by_username(&mut conn, username).await;

        match result {
            Ok(user) => match validate(user) {
                Ok(_) => return Ok("Login success."),
                Err(e) => return Err(e),
            },
            Err(NotFound) => {
                return Err(AppError::BadRequest {
                    field: GENERIC_BAD_REQUEST.to_string(),
                });
            }
            Err(_) => return Err(AppError::InternalError),
        }
    }

    if let Some(email) = &credentials.email {
        let mut conn = state.db_connection().await?;

        let result = User::find_by_email(&mut conn, email).await;

        match result {
            Ok(user) => match validate(user) {
                Ok(_) => return Ok("Login success."),
                Err(e) => return Err(e),
            },
            Err(NotFound) => {
                return Err(AppError::BadRequest {
                    field: GENERIC_BAD_REQUEST.to_string(),
                });
            }
            Err(_) => return Err(AppError::InternalError),
        }
    }

    Err(AppError::BadRequest {
        field: GENERIC_BAD_REQUEST.to_string(),
    })
}
