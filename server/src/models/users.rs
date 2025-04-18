use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Deserialize, Serialize)]
pub struct NewUserDetails {
    pub display_name: Option<String>,
    pub password: Option<String>
}

#[typeshare]
#[derive(Deserialize, Serialize)]
pub struct AddUser {
    pub username: String,
    pub password: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
}

pub struct Image {
    pub filename: String,
    
}