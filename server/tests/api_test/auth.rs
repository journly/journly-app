use journaly_server::{handlers::auth::LoginCredentials, models::api::users::CreateUser};
use reqwest::StatusCode;

use crate::spawn_app;

#[actix_rt::test]
pub async fn login_with_valid_credentials() {
    let address = spawn_app().await;

    // create user
    let client = reqwest::Client::new();

    let username = "stinkybuttman".to_string();

    let password = "thisisagreatpassword123".to_string();

    let new_user = CreateUser {
        username: username.clone(),
        password: password.clone(),
    };

    client
        .post(format!("{}/api/users", address))
        .json(&new_user)
        .send()
        .await
        .expect("Request to POST '/api/users' could not be resolved");

    // login
    let credentials = LoginCredentials {
        username: Some(username),
        email: None,
        password,
    };

    let response = client
        .post(format!("{}/auth/login", address))
        .json(&credentials)
        .send()
        .await
        .expect("Request to POST '/auth/login' could not be resolved");

    assert_eq!(response.status(), StatusCode::OK);
}

#[actix_rt::test]
pub async fn login_with_invalid_credentials() {
    let address = spawn_app().await;

    // create user
    let client = reqwest::Client::new();

    let new_user = CreateUser {
        username: "stinkybuttman".to_string(),
        password: "thisisagreatpassword123".to_string(),
    };

    client
        .post(format!("{}/api/users", address))
        .json(&new_user)
        .send()
        .await
        .expect("Request to POST '/api/users' could not be resolved");

    // login
    let credentials = LoginCredentials {
        username: None,
        email: Some("this_user_does_not_exist@email.com".to_string()),
        password: "password".to_string(),
    };

    let response = client
        .post(format!("{}/auth/login", address))
        .json(&credentials)
        .send()
        .await
        .expect("Request to POST '/auth/login' could not be resolved");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
