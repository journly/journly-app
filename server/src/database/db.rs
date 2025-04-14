use std::ops::DerefMut;
use deadpool_postgres::{Client, Config, Pool};
use dotenvy::dotenv;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::NoTls;

use crate::errors::MyError;

use crate::models::user::User;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./src/database/migrations");
}

pub async fn connection_builder(pg: Config) -> Result<Pool, MyError> {
    dotenv().ok();

    let pool = pg.create_pool(None, NoTls).expect("Failed to connect to DB");

    let mut conn = pool.get().await.unwrap();

    let client = conn.deref_mut().deref_mut();

    let report = embedded::migrations::runner().run_async(client).await.expect("Migration failed");

    println!("Migration report: {report:?}");

    Ok(pool)
}

pub async fn get_users(client: &Client) -> Result<Vec<User>, MyError> {
    let stmt = include_str!("./sql/get_users.sql");
    let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();

    let results = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>();

    Ok(results)
}

pub async fn add_user(client: &Client, user_info: User) -> Result<User, MyError> {
    let _stmt = include_str!("./sql/add_user.sql");
    let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &user_info.id,
                &user_info.email,
                &user_info.first_name,
                &user_info.last_name,
                &user_info.username,
            ],
        )
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound) // more applicable for SELECTs
}