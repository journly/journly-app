use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;

use crate::{errors::MyError, models::User};

pub async fn get_users(client: &Client) -> Result<Vec<User>, MyError> {
    let stmt = include_str!("../database/sql/get_users.sql");
    let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();

    Ok(
        client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
    )
}

pub async fn add_user(client: &Client, mut user_info: User) -> Result<User, MyError> {
    let stmt = include_str!("../database/sql/add_user.sql");

    user_info.id = Uuid::new_v4();

    let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &user_info.id,
                &user_info.username,
                &user_info.email,
                &user_info.password_hash,
                &user_info.profile_picture_url,
            ],
        )
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound) // more applicable for SELECTs
}

pub async fn get_user(client: &Client, user_id: Uuid) -> Result<User, MyError> {
    let stmt = include_str!("../database/sql/get_user.sql");
    let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = stmt.replace("$user_id", &user_id.to_string());
    let stmt = client.prepare(&stmt).await.unwrap();

    client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound) 
}

pub async fn update_user(client: &Client, new_user_info: User) -> Result<User, MyError> {
    Ok()
}

pub async fn delete_user(client: &Client, user_id: Uuid) -> Result<User, MyError> {
    Ok()
}