use std::str::FromStr;

use crate::data_tests::init_db_context;
use chrono::NaiveDate;
use journaly_server::models::api::dates::Dates;
use uuid::Uuid;

const TEST_TRIP_ID: &str = "c8381024-3f79-4a10-b5fe-06dc24e74bdc";
const TEST_USER1_ID: &str = "612e21ed-869b-4130-bb72-fc7549f93609";
const TEST_USER2_ID: &str = "3b918c91-0cf2-4788-93f9-10d1f77ec3a9";

#[actix_rt::test]
async fn get_all_trips_works() {
    let db = init_db_context().await;

    let result = db.trips.get_all_trips().await;

    assert!(!result.unwrap().is_empty());
}

#[actix_rt::test]
async fn get_single_trip_works() {
    let db = init_db_context().await;

    let trip_id = Uuid::from_str(TEST_TRIP_ID).unwrap();

    let result = db.trips.get_trip(trip_id).await;

    assert!(result.is_ok());
}

#[actix_rt::test]
async fn add_trip_works() {
    let db = init_db_context().await;

    // pre-populated user
    let test_user_id = Uuid::from_str(TEST_USER1_ID).unwrap();

    let result = db.trips.add_trip(test_user_id).await;

    assert!(result.is_ok());
}

#[actix_rt::test]
async fn update_trip_title_works() {
    let db = init_db_context().await;

    let trip_id = Uuid::from_str(TEST_TRIP_ID).unwrap();

    let new_title = "new title".to_string();

    let result = db.trips.update_trip_title(trip_id, new_title).await;

    assert!(result.is_ok());
}

#[actix_rt::test]
async fn update_trip_owner_works() {
    let db = init_db_context().await;

    let trip_id = Uuid::from_str(TEST_TRIP_ID).unwrap();

    let new_owner = Uuid::from_str(TEST_USER2_ID).unwrap();

    let result = db.trips.update_trip_owner(trip_id, new_owner).await;

    assert!(result.is_ok());
}

#[actix_rt::test]
async fn update_trip_dates_works() {
    let db = init_db_context().await;

    let trip_id = Uuid::from_str(TEST_TRIP_ID).unwrap();

    let start_date = NaiveDate::parse_from_str("2025-05-05", "%Y-%m-%d").unwrap();
    let end_date = NaiveDate::parse_from_str("2025-05-20", "%Y-%m-%d").unwrap();

    let new_dates = Dates {
        start_date: Some(start_date),
        end_date: Some(end_date),
    };

    let result = db.trips.update_trip_dates(trip_id, new_dates).await;

    assert!(result.is_ok());
}

#[actix_rt::test]
async fn update_trip_image_url_works() {
    let db = init_db_context().await;

    let trip_id = Uuid::from_str(TEST_TRIP_ID).unwrap();

    let new_image_url = "www.images.com/new_image_url.png".to_string();

    let result = db
        .trips
        .update_trip_image_url(trip_id, Some(new_image_url))
        .await;

    assert!(result.is_ok());
}
