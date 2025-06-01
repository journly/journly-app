use reqwest::StatusCode;
use uuid::Uuid;

use crate::spawn_app;
use journly_server::controllers::user::{GetUsersResponse};

#[actix_rt::test]
pub async fn get_users_returns_list() {
    let address = spawn_app().await;

    let response = reqwest::get(format!("{}/api/v1/users", address))
        .await
        .expect("Request to GET '/users' failed to resolve.");

    assert_eq!(response.status(), StatusCode::OK);

    let text = response.text().await.unwrap();

    serde_json::from_str::<GetUsersResponse>(&text).expect("Failed to parse GET '/users' response body.");
}

#[actix_rt::test]
pub async fn get_user_with_valid_id_returns_user() {
    let address = spawn_app().await;

    let client_id = "612e21ed-869b-4130-bb72-fc7549f93609";

    let response = reqwest::get(format!("{}/api/users/{}", address, client_id))
        .await
        .expect("Request to GET '/users/{user_id}' failed to resolve.");

    assert_eq!(response.status(), StatusCode::OK);

    let text = response.text().await.unwrap();

    serde_json::from_str::<User>(&text).expect("Failed to parse GET '/users/{id}' response body.");
}

#[actix_rt::test]
pub async fn get_user_with_invalid_id_returns_404_not_found() {
    let address = spawn_app().await;

    let invalid_client_id = Uuid::new_v4();

    let response = reqwest::get(format!("{}/api/users/{}", address, invalid_client_id))
        .await
        .expect("Request to GET '/users/{user_id}' failed to resolve.");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
pub async fn create_user_with_valid_params() {
    let address = spawn_app().await;

    let new_user = CreateUser {
        username: "test_new_user".to_string(),
        password: "password_test".to_string(),
    };

    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/api/users", address))
        .json(&new_user)
        .send()
        .await
        .expect("Request to POST '/users' failed to resolve.");

    assert_eq!(response.status(), StatusCode::OK);

    let response_body = response.text().await.unwrap();

    let response_body = serde_json::from_str::<User>(&response_body)
        .expect("Failed to parse POST '/users' response body");

    let get_response = reqwest::get(format!("{}/api/users/{}", address, response_body.id))
        .await
        .expect("Request to GET '/users/{user_id}' failed to resolve.");

    assert_eq!(get_response.status(), StatusCode::OK);
}

#[actix_rt::test]
pub async fn create_user_with_invalid_params() {
    let address = spawn_app().await;

    let new_user = CreateUser {
        username: "fdsa fds dsf sdff sfsd fasd@$!Q) +_".to_string(),
        password: "3249 fa 0$)@%_! ().. -~~~".to_string(),
    };

    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/api/users", address))
        .json(&new_user)
        .send()
        .await
        .expect("Request to POST '/users' failed to resolve.");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[actix_rt::test]
pub async fn update_user_display_name() {
    let address = spawn_app().await;

    let display_name = "new display name".to_string();

    let new_display_name = UserDisplayName {
        display_name: display_name.clone(),
    };

    let client_id = "612e21ed-869b-4130-bb72-fc7549f93609";

    let client = reqwest::Client::new();

    let response = client
        .put(format!("{}/api/users/{}/display_name", address, client_id))
        .json(&new_display_name)
        .send()
        .await
        .expect("Request to PUT '/users/{user_id}/display_name' failed to resolve.");

    assert_eq!(response.status(), StatusCode::OK);

    let response = reqwest::get(format!("{}/api/users/{}", address, client_id))
        .await
        .expect("Request to GET '/users/{user_id}' failed to resolve.");

    assert_eq!(response.status(), StatusCode::OK);

    let response_body = response.text().await.unwrap();

    let response_body = serde_json::from_str::<User>(&response_body)
        .expect("Failed to parse GET '/users/{user_id}' response body.");

    assert_eq!(response_body.display_name, Some(display_name));
}

#[actix_rt::test]
pub async fn update_user_email() {
    let address = spawn_app().await;

    let email = "newemail@journaly.com".to_string();

    let new_email = UserEmail {
        email: email.clone(),
    };

    let client_id = "612e21ed-869b-4130-bb72-fc7549f93609";

    let client = reqwest::Client::new();

    let response = client
        .put(format!("{}/api/users/{}/email", address, client_id))
        .json(&new_email)
        .send()
        .await
        .expect("Request to PUT '/users/{user_id}/email' failed to resolve.");

    assert_eq!(response.status(), StatusCode::OK);

    let response = reqwest::get(format!("{}/api/users/{}", address, client_id))
        .await
        .expect("Request to GET '/users/{user_id}' failed to resolve.");

    assert_eq!(response.status(), StatusCode::OK);

    let response_body = response.text().await.unwrap();

    let response_body = serde_json::from_str::<User>(&response_body)
        .expect("Failed to parse GET '/users/{user_id}' response body.");

    assert_eq!(response_body.email, Some(email));
}
