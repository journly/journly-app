use dotenvy::dotenv;
use jsonwebtoken::Algorithm;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
