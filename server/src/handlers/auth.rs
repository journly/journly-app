use actix_identity::Identity;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{models::api::users::User, AppData};

#[derive(Deserialize, Serialize, ToSchema, Debug, Clone)]
pub struct LoginCredentials {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

pub enum ValidateResult<'a> {
    Error(&'a str),
    NotFound(&'a str),
    Found(&'a str),
}

#[utoipa::path(
    tag = "test",
    post,
    path = "/api/auth/login",
    responses(
        (status = 200, description = "Login was successful", body = str)
    ),
    params(
        ("id" = u64, Path, description = "id to test shit"),
    )
)]
pub async fn login(
    credentials: web::Json<LoginCredentials>,
    req: HttpRequest,
    app_data: web::Data<AppData>,
) -> impl Responder {
    let validate = |user: User| -> ValidateResult {
        let salt = match SaltString::from_b64(
            &general_purpose::STANDARD_NO_PAD.encode(user.password_salt),
        ) {
            Ok(res) => res,
            _ => return ValidateResult::Error("Salt string failed to be encoded."),
        };

        let argon2 = Argon2::default();

        let password_hash = match argon2.hash_password(credentials.password.as_bytes(), &salt) {
            Ok(hash) => hash.to_string(),
            _ => return ValidateResult::Error("Password hash failed."),
        };

        if password_hash == user.password_hash {
            let _ = Identity::login(&req.extensions(), user.id.to_string().to_owned());

            return ValidateResult::Found("User was found.");
        };

        ValidateResult::NotFound("User could not be found.")
    };

    if let Some(username) = &credentials.username {
        let result = app_data
            .db
            .users
            .get_user_by_username(username.to_string())
            .await;

        match result {
            Ok(user) => match validate(user) {
                ValidateResult::Found(s) => return HttpResponse::Ok().body(s),
                ValidateResult::NotFound(s) => return HttpResponse::BadRequest().body(s),
                ValidateResult::Error(_) => return HttpResponse::InternalServerError().into(),
            },
            Err(_) => return HttpResponse::BadRequest().body("User not found."),
        }
    }

    if let Some(email) = &credentials.email {
        let result = app_data.db.users.get_user_by_email(email.to_string()).await;

        match result {
            Ok(user) => match validate(user) {
                ValidateResult::Found(s) => return HttpResponse::Ok().body(s),
                ValidateResult::NotFound(s) => return HttpResponse::BadRequest().body(s),
                ValidateResult::Error(_) => return HttpResponse::InternalServerError().into(),
            },
            Err(_) => return HttpResponse::BadRequest().body("User not found."),
        }
    }

    HttpResponse::BadRequest().body("Check your username/email and password")
}
