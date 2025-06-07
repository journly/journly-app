use std::str::FromStr;

use chrono::NaiveDate;
use journly_server::{
    controllers::{
        helper::OkResponse,
        trip::{CreateTrip, GetTripResponse, GetTripsResponse},
    },
    models::api::dates::Dates,
};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::spawn_app;

#[actix_rt::test]
pub async fn get_trips_returns_list() {
    let address = spawn_app().await;

    let response = reqwest::get(format!("{}/api/v1/trips", address))
        .await
        .expect("Request could not be resolved.");

    assert_eq!(response.status(), StatusCode::OK);

    let text = response.text().await.unwrap();

    serde_json::from_str::<GetTripsResponse>(&text)
        .expect("Failed to parse get_trips return value.");
}

#[actix_rt::test]
pub async fn get_trip_with_valid_id_returns_trip() {
    let address = spawn_app().await;

    let trip_id = "c8381024-3f79-4a10-b5fe-06dc24e74bdc";

    let response = reqwest::get(format!("{}/api/v1/trips/{}", address, trip_id))
        .await
        .expect("Request could not be resolved.");

    assert_eq!(response.status(), StatusCode::OK);

    let text = response.text().await.unwrap();

    serde_json::from_str::<GetTripResponse>(&text)
        .expect("Failed to parse '/trips' response body.");
}

#[actix_rt::test]
pub async fn get_trip_with_invalid_id_returns_404_not_found() {
    let address = spawn_app().await;

    let trip_id = "invalid-trip-id";

    let response = reqwest::get(format!("{}/api/v1/trips/{}", address, trip_id))
        .await
        .expect("Request could not be resolved.");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
pub async fn create_trip_with_valid_information_returns_trip() {
    let address = spawn_app().await;

    let body = CreateTrip {
        user_id: Uuid::from_str("612e21ed-869b-4130-bb72-fc7549f93609").unwrap(),
        title: Some("New Trip".to_string()),
        start_date: None,
        end_date: None,
    };

    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/api/v1/trips", address))
        .json(&body)
        .send()
        .await
        .expect("Request could not be resolved.");

    assert_eq!(response.status(), StatusCode::OK);

    let response = reqwest::get(format!("{}/api/v1/trips", address))
        .await
        .expect("Request could not be resolved.");

    let text = response.text().await.unwrap();

    let trips = serde_json::from_str::<GetTripsResponse>(&text)
        .unwrap()
        .trips;

    assert!(
        trips
            .iter()
            .find(|trip| trip.title == "New Trip".to_string())
            .is_some()
    )
}

#[actix_rt::test]
pub async fn create_trip_with_invalid_information_returns_400_bad_request() {
    let address = spawn_app().await;

    let body = CreateTrip {
        user_id: Uuid::new_v4(),
        title: None,
        start_date: None,
        end_date: None,
    };

    let client = reqwest::Client::new();

    let url = format!("{}/api/v1/trips", address);

    // invalid owner_id
    let bad_resp1 = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .expect("Request could not be resolved.");

    assert_eq!(bad_resp1.status(), StatusCode::BAD_REQUEST);

    // no request body
    let bad_resp2 = client
        .post(url)
        .send()
        .await
        .expect("Request could not be resolved.");

    assert_eq!(bad_resp2.status(), StatusCode::BAD_REQUEST);
}
