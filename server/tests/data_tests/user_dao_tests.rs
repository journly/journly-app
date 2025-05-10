use std::str::FromStr;

use uuid::Uuid;

use super::init_db_context;

const TEST_USER1_ID: &str = "612e21ed-869b-4130-bb72-fc7549f93609";

#[actix_rt::test]
pub async fn get_all_users_works() {
    let db = init_db_context().await;

    let result = db.users.get_users().await;

    assert!(!result.unwrap().is_empty());
}

#[actix_rt::test]
pub async fn get_single_user_works() {
    let db = init_db_context().await;

    let user_id = Uuid::from_str(TEST_USER1_ID).unwrap();

    let result = db.users.get_user_by_id(user_id).await;

    assert!(result.is_ok());
}

#[actix_rt::test]
pub async fn add_user_works() {
    let db =init_db_context().await;

    let username = "NewUser".to_string();
    let password = "12345".to_string();

    let result = db.users.add_user(username, password).await;

    assert!(result.is_ok());
} 
