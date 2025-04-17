use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;
use argon2::{ password_hash::{ rand_core::OsRng, PasswordHasher, SaltString }, Argon2 };
use crate::{errors::MyError, models::{schema::User, users::NewUserDetails}};

pub async fn get_users(client: &Client) -> Result<Vec<User>, MyError> {
    let stmt = include_str!("./sql/user_controllers/get_users.sql");
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
    let stmt = include_str!("./sql/user_controllers/add_user.sql");
    let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();
    
    user_info.id = Uuid::new_v4();
    client
        .query(
            &stmt,
            &[
                &user_info.id,
                &user_info.display_name,
                &user_info.username,
                &user_info.email,
                &user_info.password_hash,
                &user_info.profile_picture_id,
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
    let stmt = include_str!("./sql/user_controllers/get_user.sql");
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

pub async fn update_user_details(client: &Client, new_user_details: NewUserDetails, user_id: Uuid) -> Result<User, MyError> {
    let mut updates: Vec<String> = Vec::new(); 

    if let Some(display_name) = new_user_details.display_name {
        updates.push(format!("display_name = {}", display_name).to_string());
    }

    if let Some(password) = new_user_details.password {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        match argon2.hash_password(password.as_bytes(), &salt) {
            Ok(hash) => updates.push(format!("password_hash = {}", hash.to_string())),
            Err(_) => return Err(MyError::InternalError)
        }
    }
    
    let stmt = include_str!("./sql/user_controllers/update_user.sql");
    let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = stmt.replace("$user_id", &user_id.to_string());
    let stmt = stmt.replace("$new_info", &updates.join(", "));
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

pub async fn delete_user(client: &Client, user_id: Uuid) -> Result<User, MyError> {
    let stmt = include_str!("./sql/user_controllers/update_user.sql");
    let stmt = stmt.replace("$user_id", &user_id.to_string());
    let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();

    client.
        query(&stmt, &[])
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound)
}