use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Deserialize, Serialize)]
pub struct NewUserDetails {
    pub display_name: Option<String>,
    pub password: Option<String>
}