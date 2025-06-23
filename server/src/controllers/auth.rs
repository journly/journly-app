use super::helper::OkResponse;
use crate::{
    app::AppState,
    auth::{AuthenticatedUser, create_token},
    google_oauth::{get_google_user, request_token},
    models::{
        refresh_tokens::RefreshToken,
        user::{NewUser, User},
    },
    util::{
        auth::{is_valid_email, is_valid_username},
        errors::{AppError, AppResult},
    },
    views::EncodableUser,
};
use actix_web::{
    HttpResponse,
    http::header::LOCATION,
    web::{self, Json},
};
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use base64::{Engine, engine::general_purpose};
use diesel::result::Error::NotFound;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

const AUTH: &str = "authentication";

pub enum ValidateResult<'a> {
    Error(&'a str),
    Found(&'a str),
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterUserBody {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct RegisterUserResponse {
    pub user: EncodableUser,
}

#[utoipa::path(
    tag = AUTH,
    post,
    path = "/api/v1/auth/register",
    responses(
        (status = 200, description = "Successfully registered", body = RegisterUserResponse),
        (status = 400, description = "Invalid registration details")
    ),
)]
pub async fn register_user(
    body: web::Json<RegisterUserBody>,
    state: web::Data<AppState>,
) -> AppResult<Json<RegisterUserResponse>> {
    let salt = SaltString::generate(&mut OsRng);
    let salt_bytes: Vec<u8> = general_purpose::STANDARD_NO_PAD
        .decode(salt.as_str())
        .unwrap();

    let argon2 = Argon2::default();

    let password_hash: String;
    if let Ok(hash) = argon2.hash_password(body.password.as_bytes(), &salt) {
        password_hash = hash.to_string();
    } else {
        return Err(AppError::InternalError);
    }

    // check username validity
    if !is_valid_username(&body.username) {
        return Err(AppError::BadRequest(
            "Username cannot contain spaces or non-alphanumeric characters".to_string(),
        ));
    }

    // check email validity
    if !is_valid_email(&body.email) {
        return Err(AppError::BadRequest("Malformed email address".to_string()));
    }

    let new_user = NewUser {
        username: Some(&body.username),
        email: Some(&body.email),
        password_hash: Some(&password_hash),
        password_salt: Some(&salt_bytes),
        avatar: None,
        provider: Some("local"),
    };

    let mut conn = state.db_connection().await?;

    let result = new_user.insert(&mut conn).await;

    match result {
        Ok(user) => Ok(Json(RegisterUserResponse {
            user: EncodableUser::from(user),
        })),
        Err(_) => Err(AppError::BadRequest("Email already exists".to_string())),
    }
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
    security(
        ("jwt" = [])
    )
)]
pub async fn get_me(
    authenticated: AuthenticatedUser,
    state: web::Data<AppState>,
) -> AppResult<Json<GetMeResponse>> {
    let user_id = authenticated.user_id;

    let mut conn = state.db_connection().await?;

    match User::find(&mut conn, &user_id).await {
        Ok(user) => Ok(Json(GetMeResponse {
            user: EncodableUser::from(user),
        })),
        Err(_) => Err(AppError::NotFound),
    }
}

#[derive(Deserialize, Serialize, ToSchema, Debug, Clone)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
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
                let access_token_secret = &state.config.jwt_config.access_secret;
                let access_token_expiration = state.config.jwt_config.access_token_expiration;

                let refresh_token_secret = &state.config.jwt_config.refresh_secret;
                let refresh_token_expiration = state.config.jwt_config.refresh_token_expiration;

                let access_token = create_token(
                    &user.id,
                    access_token_secret,
                    access_token_expiration,
                    &user.role,
                );
                let refresh_token =
                    create_token(&user.id, refresh_token_secret, refresh_token_expiration, "");

                return match RefreshToken::create(
                    &mut conn,
                    &refresh_token,
                    &user.id,
                    refresh_token_expiration,
                )
                .await
                {
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
pub struct QueryCode {
    pub code: String,
    pub state: String,
}

#[utoipa::path(
    tag = AUTH,
    get,
    path = "/api/v1/auth/google",
    responses(
        (status = 200, description = "Successfully logged in using Google OAuth 2.0", body = LoginResponse)
    )
)]
pub async fn google_oauth(
    query: web::Query<QueryCode>,
    state: web::Data<AppState>,
) -> AppResult<HttpResponse> {
    let query_code = &query.code;
    let query_state = &query.state;

    if query_code.is_empty() {
        return Err(AppError::Unauthorized);
    }

    let token_response = request_token(query_code.as_str(), &state).await;
    if token_response.is_err() {
        return Err(AppError::BadGateway);
    }

    let token_response = token_response.unwrap();
    let google_user = get_google_user(&token_response.access_token, &token_response.id_token).await;
    if google_user.is_err() {
        return Err(AppError::BadGateway);
    }

    let google_user = google_user.unwrap();

    let mut conn = state.db_connection().await?;

    let user_exists = User::find_by_email(&mut conn, &google_user.email).await;

    let user_id: Uuid;
    let user_role: String;

    match user_exists {
        Ok(user) => {
            user_id = user.id;

            user_role = user.role;
            Ok(())
        }
        Err(_) => {
            let new_user = NewUser {
                username: Some(&google_user.given_name),
                email: Some(&google_user.email),
                password_hash: None,
                password_salt: None,
                avatar: None,
                provider: Some("google"),
            };

            user_role = "user".to_string();

            match new_user.insert(&mut conn).await {
                Ok(user) => {
                    user_id = user.id;
                    Ok(())
                }
                Err(_) => {
                    // get rid of warning
                    user_id = Uuid::new_v4();
                    Err(AppError::InternalError)
                }
            }
        }
    }?;

    let access_token_secret = &state.config.jwt_config.access_secret;
    let access_token_expiration = state.config.jwt_config.access_token_expiration;

    let refresh_token_secret = &state.config.jwt_config.refresh_secret;
    let refresh_token_expiration = state.config.jwt_config.refresh_token_expiration;

    let access_token = create_token(
        &user_id,
        access_token_secret,
        access_token_expiration,
        &user_role,
    );
    let refresh_token = create_token(
        &user_id,
        refresh_token_secret,
        refresh_token_expiration,
        &user_role,
    );

    return match RefreshToken::create(
        &mut conn,
        &refresh_token,
        &user_id,
        refresh_token_expiration,
    )
    .await
    {
        Ok(_) => {
            let frontend_origin = &state.config.base.domain_name;

            let response_body = serde_json::to_string(&LoginResponse {
                access_token,
                refresh_token,
            })
            .unwrap();

            Ok(HttpResponse::Ok()
                .append_header((LOCATION, format!("{}{}", frontend_origin, query_state)))
                .body(response_body))
        }
        Err(_) => Err(AppError::InternalError),
    };
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
    let user_id = authenticated.user_id;

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
                let access_token = create_token(&user_id, secret, access_expiration_time, "user");

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
