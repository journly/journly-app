use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use uuid::Uuid;

use super::{dates::UpdateDates, ToSql};

#[typeshare]
#[derive(Deserialize, Serialize)]
pub struct CreateTrip {
    pub owner_id: Uuid,
}

#[typeshare]
#[derive(Deserialize, Serialize)]
pub struct UpdateTrip {
    pub title: Option<String>,
    pub owner_id: Option<Uuid>,
    pub dates: Option<UpdateDates>,
}

impl ToSql for UpdateTrip {
    fn to_sql_values(&self) -> String {
        let mut values = Vec::new();

        if let Some(title) = &self.title {
            values.push(format!("title = '{}'", title));
        }

        if let Some(owner_id) = &self.owner_id {
            values.push(format!("owner_id = '{}'", owner_id));
        }

        values.join(", ")
    }
}

// #[typeshare]
// #[derive(Deserialize, Serialize)]
// pub struct TripPageData {
//     pub title: String,
//     pub owner_id: String,
//     pub 
// }
