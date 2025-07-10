use actix_web::{FromRequest, HttpRequest, dev::Payload, web::Data};
use chrono::{Duration, TimeZone, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use serde::{Deserialize, Serialize};
use std::future::ready;
use uuid::Uuid;

use crate::{app::AppState, models::user::User, util::errors::AppError};

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
    pub user: User,
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
    type Error = AppError;
    type Future = futures_util::future::LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let server_error = Box::pin(ready(Err(AppError::InternalError)));

        let state = match req.app_data::<Data<AppState>>() {
            Some(s) => s.clone(),
            None => return server_error,
        };

        let config = state.config.clone();

        let unauthorized_error = Box::pin(ready(Err(AppError::Unauthorized)));

        let header = match req.headers().get("Authorization") {
            Some(header) => header.to_str().unwrap(),
            None => return unauthorized_error,
        };

        let token = match header.strip_prefix("Bearer ") {
            Some(token) => token,
            None => return unauthorized_error,
        };

        if let Ok(token_data) = verify_jwt(token, &config.jwt_config.access_secret) {
            return Box::pin(async move {
                let issued_at = Utc.timestamp_opt(token_data.claims.iat, 0).unwrap();
                let expiration_time = Utc.timestamp_opt(token_data.claims.exp, 0).unwrap();

                if expiration_time < Utc::now() {
                    return Err(AppError::Unauthorized);
                }

                if issued_at > Utc::now() {
                    return Err(AppError::Unauthorized);
                }

                match state.db_connection().await {
                    Ok(mut conn) => {
                        let result = User::find(&mut conn, &token_data.claims.sub).await;

                        match result {
                            Ok(user) => {
                                if user.verified || state.emails.is_none() {
                                    Ok(AuthenticatedUser {
                                        user,
                                        role: token_data.claims.role,
                                    })
                                } else {
                                    Err(AppError::UnverifiedUser(
                                        "User has not verified their email.".to_string(),
                                    ))
                                }
                            }
                            Err(_) => Err(AppError::NotFound),
                        }
                    }
                    Err(_) => Err(AppError::InternalError),
                }
            });
        }

        unauthorized_error
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
