use actix_web::web;
use reqwest::{Client, Url};
use serde::Deserialize;
use std::error::Error;

use crate::app::AppState;

#[derive(Deserialize)]
pub struct OAuthResponse {
    pub access_token: String,
    pub id_token: String,
}

#[derive(Deserialize)]
pub struct GoogleUserResult {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
}

pub async fn request_token(
    authorization_code: &str,
    data: &web::Data<AppState>,
) -> Result<OAuthResponse, Box<dyn Error>> {
    match &data.config.google_oauth {
        Some(config) => {
            let redirect_url = config.redirect_url.clone();
            let client_secret = config.client_secret.clone();
            let client_id = config.client_id.clone();
            let root_url = "https://oauth2.googleapis.com/token";

            let client = Client::new();

            let params = [
                ("grant_type", "authorization_code"),
                ("redirect_uri", redirect_url.as_str()),
                ("client_id", client_id.as_str()),
                ("code", authorization_code),
                ("client_secret", client_secret.as_str()),
            ];
            let response = client.post(root_url).form(&params).send().await?;

            if response.status().is_success() {
                let oauth_response = response.json::<OAuthResponse>().await?;
                Ok(oauth_response)
            } else {
                let message = "An error occurred while trying to retrieve access token.";
                Err(From::from(message))
            }
        }
        None => Err("Missing GoogleOAuth config".into()),
    }
}

pub async fn get_google_user(
    access_token: &str,
    id_token: &str,
) -> Result<GoogleUserResult, Box<dyn Error>> {
    let client = Client::new();
    let mut url = Url::parse("https://www.googleapis.com/oauth2/v1/userinfo").unwrap();
    url.query_pairs_mut().append_pair("alt", "json");
    url.query_pairs_mut()
        .append_pair("access_token", access_token);

    let response = client.get(url).bearer_auth(id_token).send().await?;

    eprintln!("get google user response {:?}", response);

    if response.status().is_success() {
        let user_info = response.json::<GoogleUserResult>().await?;
        Ok(user_info)
    } else {
        let message = "An error occurred while trying to retrieve user information.";
        Err(From::from(message))
    }
}
