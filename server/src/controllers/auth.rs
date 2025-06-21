use super::helper::OkResponse;
use crate::{
    app::AppState,
    auth::{AuthenticatedUser, create_token},
    models::{
        refresh_tokens::RefreshToken,
        user::{NewUser, User},
    },
    util::{
        auth::is_valid_email,
        errors::{AppError, AppResult},
    },
    views::EncodableUser,
};
use actix_web::web::{self, Json};
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use base64::{Engine, engine::general_purpose};
use diesel::result::Error::NotFound;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

const ACCESS_TOKEN_EXPIRATION: i64 = 10; // 10 mins

const REFRESH_TOKEN_EXPIRATION: i64 = 10080; // 1 week

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
pub struct QueryCode {
    pub code: String,
    pub state: String,
}

// #[utoipa::path(
//     tag = AUTH,
//     get,
//     path = "/api/v1/auth/google",
//     responses(
//         (status = 200, description = "Successfully logged in using Google OAuth 2.0", body = LoginResponse)
//     )
// )]
// pub async fn google_oauth(
//     query: web::Query<QueryCode>,
//     state: web::Data<AppState>,
// ) -> AppResult<Json<LoginResponse>> {
//     let code = &query.code;
//     let state = &query.state;
//
//     if code.is_empty() {
//         return Err(AppError::Unauthorized);
//     }
//
//     let token_response = request_token(code.as_str(), &data).await;
//     if token_response.is_err() {
//         let message = token_response.err().unwrap().to_string();
//         return Err(AppError::BadGateway);
//     }
//
//     let token_response = token_response.unwrap();
//     let google_user = get_google_user(&token_response.access_token, &token_response.id_token).await;
//     if google_user.is_err() {
//         let message = google_user.err().unwrap().to_string();
//         return Err(AppError::BadGateway);
//     }
//
//     let google_user = google_user.unwrap();
//
//
//     let mut conn = state.db_connection().await?;
//
//     let user_search = diesel
//
//     let user_id: String;
//
//     if user.is_some() {
//         let user = user.unwrap();
//         user_id = user.id.to_owned().unwrap();
//         user.email = email.to_owned();
//         user.photo = google_user.picture;
//         user.updatedAt = Some(Utc::now());
//     } else {
//         let datetime = Utc::now();
//         let id = Uuid::new_v4();
//         user_id = id.to_owned().to_string();
//         let user_data = User {
//             id: Some(id.to_string()),
//             name: google_user.name,
//             verified: google_user.verified_email,
//             email,
//             provider: "Google".to_string(),
//             role: "user".to_string(),
//             password: "".to_string(),
//             photo: google_user.picture,
//             createdAt: Some(datetime),
//             updatedAt: Some(datetime),
//         };
//
//         vec.push(user_data.to_owned());
//     }
//
//     let jwt_secret = data.env.jwt_secret.to_owned();
//     let now = Utc::now();
//     let iat = now.timestamp() as usize;
//     let exp = (now + Duration::minutes(data.env.jwt_max_age)).timestamp() as usize;
//     let claims: TokenClaims = TokenClaims {
//         sub: user_id,
//         exp,
//         iat,
//     };
//
//     let token = encode(
//         &Header::default(),
//         &claims,
//         &EncodingKey::from_secret(jwt_secret.as_ref()),
//     )
//     .unwrap();
//
//     let cookie = Cookie::build("token", token)
//         .path("/")
//         .max_age(ActixWebDuration::new(60 * data.env.jwt_max_age, 0))
//         .http_only(true)
//         .finish();
//
//     let frontend_origin = data.env.client_origin.to_owned();
//     let mut response = HttpResponse::Found();
//     response.append_header((LOCATION, format!("{}{}", frontend_origin, state)));
//     response.cookie(cookie);
//     response.finish()
// }

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
