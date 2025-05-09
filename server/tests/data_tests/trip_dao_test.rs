use std::str::FromStr;

use crate::init_db_context;
use uuid::Uuid;

#[actix_rt::test]
async fn get_all_trips_works() {
    let db = init_db_context().await;

    let result = db.trips.get_all_trips().await;

    assert!(result.is_ok());
}

#[actix_rt::test]
async fn get_single_trip_works() {
    let app = DaoTest::new().await;
}

#[actix_rt::test]
async fn add_trip_works() {
    let db = init_db_context().await;
    // pre-populated user
    let test_user_id = Uuid::from_str("612e21ed-869b-4130-bb72-fc7549f93609").unwrap();

    let result = db.trips.add_trip(test_user_id);

    assert!(result.await.is_ok());
}

#[actix_rt::test]
async fn date_is_created_when_trip_is_added() {
    let app = DaoTest::new().await;
}
