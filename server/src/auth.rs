use actix_web::{
    Error, FromRequest, HttpRequest,
    dev::Payload,
    error::{ErrorInternalServerError, ErrorUnauthorized},
    web::Data,
};
use chrono::{Duration, TimeZone, Utc};
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
    pub exp: i64,
    pub iat: i64,
    pub jti: Uuid,
    pub role: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JwtConfig {
    pub access_secret: String,
    pub refresh_secret: String,
    pub algorithm: Algorithm,
    pub access_token_expiration: i64,
    pub refresh_token_expiration: i64,
}

pub fn create_token(user_id: &Uuid, secret: &str, expiration_in_mins: i64, role: &str) -> String {
    let expiration = Utc::now() + Duration::minutes(expiration_in_mins);
    let claims = Claims {
        sub: *user_id,
        exp: expiration.timestamp(),
        iat: Utc::now().timestamp(),
        jti: Uuid::new_v4(),
        role: role.to_string(),
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

pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub role: String,
}

impl AuthenticatedUser {
    pub fn is_admin(&self) -> bool {
        if self.role == "admin" {
            return true;
        }
        false
    }
}

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
                        let issued_at = Utc.timestamp_opt(token_data.claims.iat, 0).unwrap();
                        let expiration_time = Utc.timestamp_opt(token_data.claims.exp, 0).unwrap();

                        if expiration_time < Utc::now() {
                            return ready(Err(ErrorUnauthorized("Token is expired")));
                        }

                        if issued_at > Utc::now() {
                            return ready(Err(ErrorUnauthorized("Invalid token")));
                        }

                        return ready(Ok(AuthenticatedUser {
                            user_id: token_data.claims.sub,
                            role: token_data.claims.role,
                        }));
                    }
                }
            }
        }

        ready(Err(ErrorUnauthorized("Invalid or missing token")))
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::auth::verify_jwt;

    use super::create_token;

    #[test]
    fn created_token_can_be_verified_correctly() {
        let secret = "super-secret-token-secret";

        let access_token = create_token(&Uuid::new_v4(), secret, 1, "user");

        assert!(verify_jwt(&access_token, secret).is_ok())
    }

    #[test]
    fn invalid_token_is_invalid() {
        let secret = "super-secret-token-secret";

        let another_secret = "fake-secret";

        let access_token = create_token(&Uuid::new_v4(), another_secret, 1, "user");

        assert!(verify_jwt(&access_token, secret).is_err())
    }
}
