use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use typeshare::typeshare;
use utoipa::ToSchema;
use uuid::Uuid;

#[typeshare]
#[derive(Deserialize, Serialize, ToSchema, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct User {
    pub id: Uuid,
    pub display_name: Option<String>,
    pub username: String,
    pub email: Option<String>,
    pub password_hash: String,
    pub password_salt: Vec<u8>,
    pub profile_picture_url: Option<String>,
}

#[typeshare]
#[derive(Deserialize, Serialize, ToSchema)]
pub struct UserDisplayName {
    pub display_name: String,
}

#[typeshare]
#[derive(Deserialize, Serialize, ToSchema)]
pub struct UserEmail {
    pub email: String,
}

#[typeshare]
#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
}
