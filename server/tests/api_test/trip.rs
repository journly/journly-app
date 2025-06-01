use std::str::FromStr;

use chrono::NaiveDate;
use journly_server::models::api::{
    dates::Dates,
    trips::{CreateTrip, TripDetails, TripOwner, TripTitle},
};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::spawn_app;

#[actix_rt::test]
pub async fn get_trips_returns_list() {
    let address = spawn_app().await;

    let response = reqwest::get(format!("{}/api/trips", address))
        .await
        .expect("Request could not be resolved.");

    assert_eq!(response.status(), StatusCode::OK);

    let text = response.text().await.unwrap();

    serde_json::from_str::<Vec<TripDetails>>(&text)
        .expect("Failed to parse get_trips return value.");
}

#[actix_rt::test]
pub async fn get_trip_with_valid_id_returns_trip() {
    let address = spawn_app().await;

    let trip_id = "c8381024-3f79-4a10-b5fe-06dc24e74bdc";

    let response = reqwest::get(format!("{}/api/trips/{}", address, trip_id))
        .await
        .expect("Request could not be resolved.");

    assert_eq!(response.status(), StatusCode::OK);

    let text = response.text().await.unwrap();

    serde_json::from_str::<TripDetails>(&text).expect("Failed to parse '/trips' response body.");
}

#[actix_rt::test]
pub async fn get_trip_with_invalid_id_returns_404_not_found() {
    let address = spawn_app().await;

    let trip_id = "invalid-trip-id";

    let response = reqwest::get(format!("{}/api/trips/{}", address, trip_id))
        .await
        .expect("Request could not be resolved.");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
pub async fn create_trip_with_valid_information_returns_trip() {
    let address = spawn_app().await;

    let body = CreateTrip {
        owner_id: Uuid::from_str("612e21ed-869b-4130-bb72-fc7549f93609").unwrap(),
    };

    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/api/trips", address))
        .json(&body)
        .send()
        .await
        .expect("Request could not be resolved.");

    assert_eq!(response.status(), StatusCode::OK);

    let text = response.text().await.unwrap();

    serde_json::from_str::<TripDetails>(&text).expect("Failed to parse '/trips' response body.");
}

#[actix_rt::test]
pub async fn create_trip_with_invalid_information_returns_400_bad_request() {
    let address = spawn_app().await;

    let body = CreateTrip {
        owner_id: Uuid::new_v4(),
    };

    let client = reqwest::Client::new();

    let url = format!("{}/api/trips", address);

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

#[actix_rt::test]
pub async fn update_trip_dates_works() {
    let address = spawn_app().await;

    let body = Dates {
        start_date: Some(NaiveDate::from_ymd_opt(2025, 12, 12).unwrap()),
        end_date: Some(NaiveDate::from_ymd_opt(2025, 12, 15).unwrap()),
    };

    let client = reqwest::Client::new();

    let trip_id = "c8381024-3f79-4a10-b5fe-06dc24e74bdc";

    let url = format!("{}/api/trips/{}/dates", address, trip_id);

    let response = client
        .put(&url)
        .json(&body)
        .send()
        .await
        .expect("Request could not be resolved");

    assert_eq!(response.status(), StatusCode::OK);

    let response = reqwest::get(url)
        .await
        .expect("Request could not be resolved.");

    assert_eq!(response.status(), StatusCode::OK);

    let text = response.text().await.unwrap();

    serde_json::from_str::<Dates>(&text).expect("Failed to parse '/trips/dates' response body.");
}

#[actix_rt::test]
pub async fn update_trip_owner_works() {
    let address = spawn_app().await;

    let body = TripOwner {
        owner_id: Uuid::from_str("3b918c91-0cf2-4788-93f9-10d1f77ec3a9").unwrap(),
    };

    let client = reqwest::Client::new();

    let trip_id = "c8381024-3f79-4a10-b5fe-06dc24e74bdc";

    let url = format!("{}/api/trips/{}/owner", address, trip_id);

    let response = client
        .put(&url)
        .json(&body)
        .send()
        .await
        .expect("Request could not be resolved.");

    assert_eq!(response.status(), StatusCode::OK);

    let text = response.text().await.unwrap();

    serde_json::from_str::<TripOwner>(&text).expect("Failed to parse '/trips/owner' response body");
}

#[actix_rt::test]
pub async fn update_trip_title_works() {
    let address = spawn_app().await;

    let body = TripTitle {
        title: "New Title".to_string(),
    };

    let client = reqwest::Client::new();

    let trip_id = "c8381024-3f79-4a10-b5fe-06dc24e74bdc";

    let url = format!("{}/api/trips/{}/title", address, trip_id);

    let response = client
        .put(&url)
        .json(&body)
        .send()
        .await
        .expect("Request could not be resolved.");

    assert_eq!(response.status(), StatusCode::OK);

    let text = response.text().await.unwrap();

    serde_json::from_str::<TripTitle>(&text).expect("Failed to parse '/trips/title' response body");
}
