use chrono::{Duration, Utc};
use dotenvy::{dotenv, var};
use hmac::{Hmac, Mac};
use jwt::{Header, SignWithKey, Token};
use serde::{Deserialize, Serialize};
use sha2::Sha384;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
}

pub fn create_access_token(user_id: Uuid) -> String {
    let key = get_secret_key();

    let expiration = Utc::now() + Duration::minutes(15);

    let claims = Claims {
        sub: user_id,
        exp: expiration.timestamp() as usize,
        iat: Utc::now().timestamp() as usize,
    };

    let token = Token::new(Header::default(), claims)
        .sign_with_key(&key)
        .unwrap();

    token.into()
}

pub fn create_refresh_token(user_id: Uuid) -> String {
    let key = get_secret_key();

    let expiration = Utc::now() + Duration::days(7);

    let claims = Claims {
        sub: user_id,
        exp: expiration.timestamp() as usize,
        iat: Utc::now().timestamp() as usize,
    };

    let token = Token::new(Header::default(), claims)
        .sign_with_key(&key)
        .unwrap();

    token.into()
}

pub fn get_secret_key() -> Hmac<Sha384> {
    dotenv().ok();

    let secret: String = var("JWT_SECRET").expect("JWT_SECRET is required.");

    let secret = secret.as_bytes();

    Hmac::new_from_slice(secret).unwrap()
}
