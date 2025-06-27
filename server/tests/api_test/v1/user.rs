use std::panic::AssertUnwindSafe;

use crate::{api_test::util::AuthHeader, spawn_app};
use futures::FutureExt;
use journly_server::{
    controllers::{
        auth::{GetMeResponse, LoginCredentials, LoginResponse, RegisterUserBody},
        user::{GetUserResponse, GetUsersResponse, PasswordUpdateRequest, UpdateInformationBody},
    },
    views::EncodableUser,
};
use reqwest::{Client, StatusCode};
use uuid::Uuid;

const TEST_EMAIL: &str = "testuser123@email.com";
const TEST_PASSWORD: &str = "password12345";

pub struct TestUser {
    pub user: EncodableUser,
    pub access_token: String,
}

pub async fn setup_test_user(address: &String) -> TestUser {
    let client = Client::new();

    let new_user = RegisterUserBody {
        username: "".to_string(),
        email: TEST_EMAIL.to_string(),
        password: TEST_PASSWORD.to_string(),
    };

    client
        .post(format!("{address}/api/v1/auth/register"))
        .json(&new_user)
        .send()
        .await
        .expect("Failed to register test user account.");

    let creds = LoginCredentials {
        email: TEST_EMAIL.to_string(),
        password: TEST_PASSWORD.to_string(),
    };

    let login_response = client
        .post(format!("{address}/api/v1/auth/login"))
        .json(&creds)
        .send()
        .await
        .expect("Could not log in to test user account");

    let login_resp = login_response.text().await.unwrap();

    let tokens = serde_json::from_str::<LoginResponse>(&login_resp).unwrap();

    let auth_header = AuthHeader::new(&tokens.access_token);

    let response = client
        .get(format!("{address}/api/v1/auth/me"))
        .header(auth_header.header_name, auth_header.header_value)
        .send()
        .await
        .expect("Failed to get me.");

    let response_body = response.text().await.unwrap();

    let user = serde_json::from_str::<GetMeResponse>(&response_body).unwrap();

    TestUser {
        user: user.user,
        access_token: tokens.access_token,
    }
}

#[actix_rt::test]
pub async fn get_users_returns_list() {
    let test_app = spawn_app().await;

    let result = AssertUnwindSafe(async {
        let address = test_app.address.clone();
        let access_token = test_app.access_token.clone();

        let client = Client::new();

        let auth_header = AuthHeader::new(&access_token);

        let response = client
            .get(format!("{address}/api/v1/users"))
            .header(auth_header.header_name, auth_header.header_value)
            .send()
            .await
            .expect("Request to GET '/users' failed to resolve");

        assert_eq!(response.status(), StatusCode::OK);

        let text = response.text().await.unwrap();

        serde_json::from_str::<GetUsersResponse>(&text)
            .expect("Failed to parse GET '/users' response body.");
    })
    .catch_unwind()
    .await;

    test_app.cleanup().await;

    if result.is_err() {
        panic!("");
    }
}

#[actix_rt::test]
pub async fn get_user_with_valid_id_returns_user() {
    let test_app = spawn_app().await;

    let result = AssertUnwindSafe(async {
        let address = test_app.address.clone();
        let access_token = test_app.access_token.clone();
        let client_id = "612e21ed-869b-4130-bb72-fc7549f93609";

        let client = Client::new();

        let auth_header = AuthHeader::new(&access_token);

        let response = client
            .get(format!("{address}/api/v1/users/{client_id}"))
            .header(auth_header.header_name, auth_header.header_value)
            .send()
            .await
            .expect("Request to GET '/users/{user_id}' failed to resolve.");

        assert_eq!(response.status(), StatusCode::OK);

        let text = response.text().await.unwrap();

        serde_json::from_str::<GetUserResponse>(&text)
            .expect("Failed to parse GET '/users/{id}' response body.");
    })
    .catch_unwind()
    .await;

    test_app.cleanup().await;

    if result.is_err() {
        panic!("");
    }
}

#[actix_rt::test]
pub async fn get_user_with_invalid_id_returns_404_not_found() {
    let test_app = spawn_app().await;

    let result = AssertUnwindSafe(async {
        let address = test_app.address.clone();
        let access_token = test_app.access_token.clone();

        let invalid_client_id = Uuid::new_v4();

        let client = Client::new();

        let auth_header = AuthHeader::new(&access_token);

        let response = client
            .get(format!("{address}/api/v1/users/{invalid_client_id}"))
            .header(auth_header.header_name, auth_header.header_value)
            .send()
            .await
            .expect("Request to GET '/users/{user_id}' failed to resolve.");

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    })
    .catch_unwind()
    .await;

    test_app.cleanup().await;

    if result.is_err() {
        panic!("");
    }
}

#[actix_rt::test]
pub async fn update_user_username() {
    let test_app = spawn_app().await;

    let result = AssertUnwindSafe(async {
        let address = test_app.address.clone();
        let access_token = test_app.access_token.clone();

        let username = "new_username".to_string();

        let update_information = UpdateInformationBody {
            username: Some(username.clone()),
            email: None,
        };

        let client_id = "612e21ed-869b-4130-bb72-fc7549f93609";

        let client = reqwest::Client::new();

        let auth_header = AuthHeader::new(&access_token);

        let response1 = client
            .put(format!("{address}/api/v1/users/{client_id}"))
            .header(
                auth_header.header_name.clone(),
                auth_header.header_value.clone(),
            )
            .json(&update_information)
            .send()
            .await
            .expect("Request to PUT '/users/{user_id}' failed to resolve.");

        let response2 = client
            .get(format!("{}/api/v1/users/{}", address, client_id))
            .header(auth_header.header_name, auth_header.header_value)
            .send()
            .await
            .expect("Request to GET '/users/{user_id}' failed to resolve.");

        assert_eq!(response1.status(), StatusCode::OK);
        assert_eq!(response2.status(), StatusCode::OK);

        let response_body = response2.text().await.unwrap();

        let response_body = serde_json::from_str::<GetUserResponse>(&response_body)
            .expect("Failed to parse GET '/users/{user_id}' response body.");

        assert_eq!(response_body.user.username, username);
    })
    .catch_unwind()
    .await;

    test_app.cleanup().await;

    if result.is_err() {
        panic!("");
    }
}

#[actix_rt::test]
pub async fn update_user_email() {
    let test_app = spawn_app().await;

    let result = AssertUnwindSafe(async {
        let address = test_app.address.clone();
        let access_token = test_app.access_token.clone();

        let email = "newemail@journaly.com".to_string();

        let update_information = UpdateInformationBody {
            email: Some(email.clone()),
            username: None,
        };

        let client_id = "612e21ed-869b-4130-bb72-fc7549f93609";

        let client = reqwest::Client::new();

        let auth_header = AuthHeader::new(&access_token);

        let response1 = client
            .put(format!("{address}/api/v1/users/{client_id}"))
            .header(
                auth_header.header_name.clone(),
                auth_header.header_value.clone(),
            )
            .json(&update_information)
            .send()
            .await
            .expect("Request to PUT '/users/{user_id}' failed to resolve.");

        assert_eq!(response1.status(), StatusCode::OK);

        let response2 = client
            .get(format!("{address}/api/v1/users/{client_id}"))
            .header(auth_header.header_name, auth_header.header_value)
            .send()
            .await
            .expect("Request to GET '/users/{user_id}' failed to resolve.");

        assert_eq!(response2.status(), StatusCode::OK);

        let response_body = response2.text().await.unwrap();

        let response_body = serde_json::from_str::<GetUserResponse>(&response_body)
            .expect("Failed to parse GET '/users/{user_id}' response body.");

        assert_eq!(response_body.user.email, email);
    })
    .catch_unwind()
    .await;

    test_app.cleanup().await;

    if result.is_err() {
        panic!("");
    }
}

#[actix_rt::test]
pub async fn update_user_password_success() {
    let test_app = spawn_app().await;

    let result = AssertUnwindSafe(async {
        let address = test_app.address.clone();

        let TestUser { user, access_token } = setup_test_user(&address).await;

        let user_id = user.id;
        let original_password = TEST_PASSWORD.to_string();
        let new_password = "newPassword456!".to_string();

        let client = reqwest::Client::new();
        let auth_header = AuthHeader::new(&access_token);

        // Step 1: Send password update request
        let request_body = PasswordUpdateRequest {
            current_password: original_password.to_string(),
            new_password: new_password.to_string(),
        };

        let response = client
            .put(format!("{address}/api/v1/users/{user_id}/password"))
            .header(
                auth_header.header_name.clone(),
                auth_header.header_value.clone(),
            )
            .json(&request_body)
            .send()
            .await
            .expect("Request to PUT '/users/{user_id}/password' failed to resolve.");

        assert_eq!(response.status(), StatusCode::OK);

        // Step 2: Try logging in with new password
        let login_body = LoginCredentials {
            email: TEST_EMAIL.to_string(),
            password: new_password.to_string(),
        };

        let login_response = client
            .post(format!("{address}/api/v1/auth/login"))
            .json(&login_body)
            .send()
            .await
            .expect("Request to POST '/auth/login' failed.");

        assert_eq!(login_response.status(), StatusCode::OK);

        let login_response_body = login_response.text().await.unwrap();

        serde_json::from_str::<LoginResponse>(&login_response_body)
            .expect("Failed to parse login response body");
    })
    .catch_unwind()
    .await;

    test_app.cleanup().await;

    if result.is_err() {
        panic!("Test failed due to panic.");
    }
}
