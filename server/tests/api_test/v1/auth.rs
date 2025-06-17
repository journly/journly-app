use crate::{api_test::util::AuthHeader, spawn_app};
use futures::FutureExt;
use journly_server::controllers::{
    auth::{LoginCredentials, LoginResponse, RefreshTokenBody},
    user::CreateUserBody,
};
use reqwest::{Client, StatusCode};
use std::panic::AssertUnwindSafe;

const USERNAME: &str = "username123";
const EMAIL: &str = "testuser@email.com";
const PASSWORD: &str = "password123";

async fn auth_setup(server_addr: &str) {
    let client = Client::new();

    let test_user = CreateUserBody {
        username: USERNAME.to_string(),
        email: EMAIL.to_string(),
        password: PASSWORD.to_string(),
    };

    client
        .post(format!("{server_addr}/api/v1/users"))
        .json(&test_user)
        .send()
        .await
        .expect("Failed to create test user.");
}

#[actix_rt::test]
pub async fn login_with_valid_credentials_works() {
    let test_app = spawn_app().await;

    let result = AssertUnwindSafe(async {
        let address = test_app.address.clone();

        auth_setup(&address).await;

        let client = Client::new();

        let credentials = LoginCredentials {
            email: EMAIL.to_string(),
            password: PASSWORD.to_string(),
        };

        let response = client
            .post(format!("{address}/api/v1/auth/login"))
            .json(&credentials)
            .send()
            .await
            .expect("Request to POST '/login' failed to resolve");

        assert_eq!(response.status(), StatusCode::OK);
    })
    .catch_unwind()
    .await;

    test_app.cleanup().await;

    if result.is_err() {
        panic!("");
    }
}

#[actix_rt::test]
pub async fn login_with_invalid_credentials_returns_401() {
    let test_app = spawn_app().await;

    let result = AssertUnwindSafe(async {
        let address = test_app.address.clone();

        auth_setup(&address).await;

        let client = Client::new();

        let wrong_password_credentials = LoginCredentials {
            email: EMAIL.to_string(),
            password: "badpassword".to_string(),
        };

        let bad_password_response = client
            .post(format!("{address}/api/v1/auth/login"))
            .json(&wrong_password_credentials)
            .send()
            .await
            .expect("Request to POST '/login' failed to resolve");

        let wrong_email_credentials = LoginCredentials {
            email: "bademail@email.com".to_string(),
            password: PASSWORD.to_string(),
        };

        assert_eq!(bad_password_response.status(), StatusCode::UNAUTHORIZED);

        let bad_email_response = client
            .post(format!("{address}/api/v1/auth/login"))
            .json(&wrong_email_credentials)
            .send()
            .await
            .expect("Request to POST '/login' failed to resolve");

        assert_eq!(bad_email_response.status(), StatusCode::UNAUTHORIZED);
    })
    .catch_unwind()
    .await;

    test_app.cleanup().await;

    if result.is_err() {
        panic!("");
    }
}

#[actix_rt::test]
pub async fn logout_works() {
    let test_app = spawn_app().await;

    let result = AssertUnwindSafe(async {
        let address = test_app.address.clone();

        auth_setup(&address).await;

        let client = Client::new();

        let credentials = LoginCredentials {
            email: EMAIL.to_string(),
            password: PASSWORD.to_string(),
        };

        let response = client
            .post(format!("{address}/api/v1/auth/login"))
            .json(&credentials)
            .send()
            .await
            .expect("Request to POST '/login' failed to resolve");

        let response_body = response.text().await.unwrap();

        let tokens: LoginResponse =
            serde_json::from_str(&response_body).expect("Could not parse response body");

        let logout_req_body = RefreshTokenBody {
            refresh_token: tokens.refresh_token,
        };

        let auth_header = AuthHeader::new(&tokens.access_token);

        let response = client
            .post(format!("{address}/api/v1/auth/logout"))
            .json(&logout_req_body)
            .header(auth_header.header_name, auth_header.header_value)
            .send()
            .await
            .expect("Request to POST '/logout' failed to resolve");

        assert_eq!(response.status(), StatusCode::OK);
    })
    .catch_unwind()
    .await;

    test_app.cleanup().await;

    if result.is_err() {
        panic!("");
    }
}

#[actix_rt::test]
pub async fn logout_without_tokens_returns_401() {
    let test_app = spawn_app().await;

    let result = AssertUnwindSafe(async {
        let address = test_app.address.clone();

        auth_setup(&address).await;

        let client = Client::new();

        let credentials = LoginCredentials {
            email: EMAIL.to_string(),
            password: PASSWORD.to_string(),
        };

        client
            .post(format!("{address}/api/v1/auth/login"))
            .json(&credentials)
            .send()
            .await
            .expect("Request to POST '/login' failed to resolve");

        let response = client
            .post(format!("{address}/api/v1/auth/logout"))
            .send()
            .await
            .expect("Request to POST '/logout' failed to resolve");

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    })
    .catch_unwind()
    .await;

    test_app.cleanup().await;

    if result.is_err() {
        panic!("");
    }
}

#[actix_rt::test]
pub async fn get_access_token_with_refresh_token() {
    let test_app = spawn_app().await;

    let result = AssertUnwindSafe(async {
        let address = test_app.address.clone();

        auth_setup(&address).await;

        let client = Client::new();

        let credentials = LoginCredentials {
            email: EMAIL.to_string(),
            password: PASSWORD.to_string(),
        };

        let response = client
            .post(format!("{address}/api/v1/auth/login"))
            .json(&credentials)
            .send()
            .await
            .expect("Request to POST '/login' failed to resolve");

        let response_body = response.text().await.unwrap();

        let tokens: LoginResponse =
            serde_json::from_str(&response_body).expect("Could not parse response body");

        let refresh_req_body = RefreshTokenBody {
            refresh_token: tokens.refresh_token,
        };

        let response = client
            .post(format!("{address}/api/v1/auth/refresh"))
            .json(&refresh_req_body)
            .send()
            .await
            .expect("Request to POST '/refresh' failed to resolve");

        assert_eq!(response.status(), StatusCode::OK);
    })
    .catch_unwind()
    .await;

    test_app.cleanup().await;

    if result.is_err() {
        panic!("");
    }
}

#[actix_rt::test]
pub async fn refresh_token_invalidation_works() {
    let test_app = spawn_app().await;

    let result = AssertUnwindSafe(async {
        let address = test_app.address.clone();

        auth_setup(&address).await;

        let client = Client::new();

        let credentials = LoginCredentials {
            email: EMAIL.to_string(),
            password: PASSWORD.to_string(),
        };

        let response = client
            .post(format!("{address}/api/v1/auth/login"))
            .json(&credentials)
            .send()
            .await
            .expect("Request to POST '/login' failed to resolve");

        let response_body = response.text().await.unwrap();

        let tokens: LoginResponse =
            serde_json::from_str(&response_body).expect("Could not parse response body");

        let req_body = RefreshTokenBody {
            refresh_token: tokens.refresh_token,
        };

        let auth_header = AuthHeader::new(&tokens.access_token);

        let response = client
            .post(format!("{address}/api/v1/auth/logout"))
            .json(&req_body)
            .header(auth_header.header_name, auth_header.header_value)
            .send()
            .await
            .expect("Request to POST '/logout' failed to resolve");

        assert_eq!(response.status(), StatusCode::OK);

        let response = client
            .post(format!("{address}/api/v1/auth/refresh"))
            .json(&req_body)
            .send()
            .await
            .expect("Request to POST '/refresh' failed to resolve");

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    })
    .catch_unwind()
    .await;

    test_app.cleanup().await;

    if result.is_err() {
        panic!("");
    }
}
