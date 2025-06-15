use std::str::FromStr;

use journly_server::controllers::trip::{CreateTripBody, GetTripResponse, GetTripsResponse};
use reqwest::{Client, StatusCode};
use uuid::Uuid;

use crate::{api_test::util::AuthHeader, spawn_app};

#[actix_rt::test]
pub async fn get_trips_returns_list() {
    let test_app = spawn_app().await;
    let address = test_app.address.clone();
    let access_token = test_app.access_token.clone();

    let client = Client::new();

    let auth_header = AuthHeader::new(&access_token);

    let response = client
        .get(format!("{address}/api/v1/trips"))
        .header(auth_header.header_name, auth_header.header_value)
        .send()
        .await
        .expect("Request could not be resolved.");

    test_app.cleanup().await;

    assert_eq!(response.status(), StatusCode::OK);

    let text = response.text().await.unwrap();

    serde_json::from_str::<GetTripsResponse>(&text)
        .expect("Failed to parse get_trips return value.");
}

#[actix_rt::test]
pub async fn get_trip_with_valid_id_returns_trip() {
    let test_app = spawn_app().await;
    let address = test_app.address.clone();
    let access_token = test_app.access_token.clone();

    let trip_id = "c8381024-3f79-4a10-b5fe-06dc24e74bdc";

    let client = Client::new();

    let auth_header = AuthHeader::new(&access_token);

    let response = client
        .get(format!("{address}/api/v1/trips/{trip_id}"))
        .header(auth_header.header_name, auth_header.header_value)
        .send()
        .await
        .expect("Request could not be resolved.");

    test_app.cleanup().await;

    assert_eq!(response.status(), StatusCode::OK);

    let text = response.text().await.unwrap();

    serde_json::from_str::<GetTripResponse>(&text)
        .expect("Failed to parse '/trips' response body.");
}

#[actix_rt::test]
pub async fn get_trip_with_invalid_id_returns_404_not_found() {
    let test_app = spawn_app().await;
    let address = test_app.address.clone();
    let access_token = test_app.access_token.clone();

    let trip_id = "invalid-trip-id";

    let client = Client::new();

    let auth_header = AuthHeader::new(&access_token);

    let response = client
        .get(format!("{address}/api/v1/trips/{trip_id}"))
        .header(auth_header.header_name, auth_header.header_value)
        .send()
        .await
        .expect("Request could not be resolved.");

    test_app.cleanup().await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
pub async fn create_trip_with_valid_information_returns_trip() {
    let test_app = spawn_app().await;
    let address = test_app.address.clone();
    let access_token = test_app.access_token.clone();

    let body = CreateTripBody {
        user_id: Uuid::from_str("11111111-1111-1111-1111-111111111111").unwrap(),
        title: Some("New Trip".to_string()),
        start_date: None,
        end_date: None,
    };

    let client = reqwest::Client::new();

    let auth_header = AuthHeader::new(&access_token);

    let response1 = client
        .post(format!("{address}/api/v1/trips"))
        .header(
            auth_header.header_name.clone(),
            auth_header.header_value.clone(),
        )
        .json(&body)
        .send()
        .await
        .expect("Request could not be resolved.");

    let response2 = client
        .get(format!("{address}/api/v1/trips"))
        .header(auth_header.header_name, auth_header.header_value)
        .send()
        .await
        .expect("Request could not be resolved.");

    test_app.cleanup().await;

    let text = response2.text().await.unwrap();

    let trips = serde_json::from_str::<GetTripsResponse>(&text)
        .unwrap()
        .trips;

    assert_eq!(response1.status(), StatusCode::OK);
    assert!(
        trips
            .iter()
            .any(|trip| trip.title == Some("New Trip".to_string()))
    )
}

#[actix_rt::test]
pub async fn create_trip_with_invalid_information_returns_400_bad_request() {
    let test_app = spawn_app().await;
    let address = test_app.address.clone();
    let access_token = test_app.access_token.clone();

    let body = CreateTripBody {
        user_id: Uuid::new_v4(),
        title: None,
        start_date: None,
        end_date: None,
    };

    let client = reqwest::Client::new();

    let url = format!("{address}/api/v1/trips");

    let auth_header = AuthHeader::new(&access_token);

    // invalid owner_id
    let bad_resp1 = client
        .post(&url)
        .header(
            auth_header.header_name.clone(),
            auth_header.header_value.clone(),
        )
        .json(&body)
        .send()
        .await
        .expect("Request could not be resolved.");

    // no request body
    let bad_resp2 = client
        .post(url)
        .header(auth_header.header_name, auth_header.header_value)
        .send()
        .await
        .expect("Request could not be resolved.");

    test_app.cleanup().await;

    assert_eq!(bad_resp1.status(), StatusCode::BAD_REQUEST);
    assert_eq!(bad_resp2.status(), StatusCode::BAD_REQUEST);
}
