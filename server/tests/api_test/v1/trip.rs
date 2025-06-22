use std::panic::AssertUnwindSafe;

use futures::FutureExt;
use journly_server::controllers::trip::{CreateTripBody, GetTripResponse, GetTripsResponse};
use reqwest::{Client, StatusCode};

use crate::{api_test::util::AuthHeader, spawn_app};

#[actix_rt::test]
pub async fn get_trips_returns_list() {
    let test_app = spawn_app().await;

    let result = AssertUnwindSafe(async {
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

        assert_eq!(response.status(), StatusCode::OK);

        let text = response.text().await.unwrap();

        serde_json::from_str::<GetTripsResponse>(&text)
            .expect("Failed to parse get_trips return value.");
    })
    .catch_unwind()
    .await;

    test_app.cleanup().await;

    if result.is_err() {
        panic!("");
    }
}

#[actix_rt::test]
pub async fn get_trip_with_valid_id_returns_trip() {
    let test_app = spawn_app().await;

    let result = AssertUnwindSafe(async {
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

        assert_eq!(response.status(), StatusCode::OK);

        let text = response.text().await.unwrap();

        serde_json::from_str::<GetTripResponse>(&text)
            .expect("Failed to parse '/trips' response body.");
    })
    .catch_unwind()
    .await;

    test_app.cleanup().await;

    if result.is_err() {
        panic!("");
    }
}

#[actix_rt::test]
pub async fn get_trip_with_invalid_id_returns_404_not_found() {
    let test_app = spawn_app().await;

    let result = AssertUnwindSafe(async {
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
pub async fn create_trip_with_works() {
    let test_app = spawn_app().await;

    let result = AssertUnwindSafe(async {
        let address = test_app.address.clone();
        let access_token = test_app.access_token.clone();

        let body = CreateTripBody {
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
    })
    .catch_unwind()
    .await;

    test_app.cleanup().await;

    if result.is_err() {
        panic!("");
    }
}
