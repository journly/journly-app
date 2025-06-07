use std::str::FromStr;

use reqwest::header::{HeaderName, HeaderValue};

pub struct AuthHeader {
    pub header_name: HeaderName,
    pub header_value: HeaderValue,
}

impl AuthHeader {
    pub fn new(token: &str) -> Self {
        Self {
            header_name: HeaderName::from_str("Authorization").unwrap(),
            header_value: HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
        }
    }
}
