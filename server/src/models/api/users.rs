use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Deserialize, Serialize)]
pub struct NewUserDetails {
    pub display_name: Option<String>,
    pub password: Option<String>,
}

#[typeshare]
#[derive(Deserialize, Serialize)]
pub struct UpdateUser {
    pub display_name: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
}

impl UpdateUser {
    pub fn to_sql_values(self) -> String {
        let mut new_values: Vec<String> = Vec::new();

        if let Some(display_name) = self.display_name {
            new_values.push(format!("display_name = {}", display_name));
        }

        if let Some(password) = self.password {
            new_values.push(format!("password_hash = {}", password));
        }

        if let Some(email) = self.email {
            new_values.push(format!("email = {}", email));
        }

        return new_values.join(", ");
    }
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
