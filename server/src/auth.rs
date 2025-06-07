use actix_web::{
    Error, FromRequest, HttpRequest,
    dev::Payload,
    error::{ErrorInternalServerError, ErrorUnauthorized, InternalError},
    web::Data,
};
use chrono::{Duration, Utc};
use dotenvy::dotenv;
use futures::future::{Ready, ready};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid, // user ID
    pub exp: usize,
    pub iat: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JwtConfig {
    pub access_secret: String,
    pub refresh_secret: String,
    pub algorithm: Algorithm,
    pub enabled: bool,
}

impl Default for JwtConfig {
    fn default() -> Self {
        dotenv().ok();

        let access_secret: String = dotenvy::var("ACCESS_TOKEN_SECRET").unwrap();
        let refresh_secret: String = dotenvy::var("REFRESH_TOKEN_SECRET").unwrap();

        Self {
            access_secret,
            refresh_secret,
            algorithm: Algorithm::HS256,
            enabled: true,
        }
    }
}

impl JwtConfig {
    pub fn new(algorithm: Algorithm) -> Self {
        dotenv().ok();

        let access_secret: String = dotenvy::var("ACCESS_TOKEN_SECRET").unwrap();
        let refresh_secret: String = dotenvy::var("REFRESH_TOKEN_SECRET").unwrap();

        Self {
            access_secret,
            refresh_secret,
            algorithm,
            enabled: true,
        }
    }

    pub fn disabled() -> Self {
        Self {
            access_secret: "dummy-secret".to_string(),
            refresh_secret: "dummy-secret".to_string(),
            algorithm: Algorithm::HS256,
            enabled: false,
        }
    }
}

pub fn create_access_token(user_id: Uuid, secret: &str, expiration_in_mins: i64) -> String {
    let expiration = Utc::now() + Duration::minutes(expiration_in_mins);
    let claims = Claims {
        sub: user_id,
        exp: expiration.timestamp() as usize,
        iat: Utc::now().timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}

pub fn verify_jwt(
    token: &str,
    secret: &str,
) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
}

pub struct AuthenticatedUser(pub Uuid); // wrapper for user_id

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let state = req.app_data::<Data<AppState>>();

        if state.is_none() {
            return ready(Err(ErrorInternalServerError("Internal error")));
        }

        let config = state.unwrap().config.clone();

        if let Some(header) = req.headers().get("Authorization") {
            if let Ok(header_str) = header.to_str() {
                if let Some(token) = header_str.strip_prefix("Bearer ") {
                    if let Ok(token_data) = verify_jwt(token, &config.jwt_config.access_secret) {
                        return ready(Ok(AuthenticatedUser(token_data.claims.sub)));
                    }
                }
            }
        }

        ready(Err(actix_web::error::ErrorUnauthorized(
            "Invalid or missing token",
        )))
    }
}
