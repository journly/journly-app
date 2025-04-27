use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::ToSql;

#[typeshare]
#[derive(Deserialize, Serialize)]
pub struct UpdateDates {
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
}

impl ToSql for UpdateDates {
    fn to_sql_values(&self) -> String {
        let mut values = Vec::new();

        if let Some(start_date) = &self.start_date {
            values.push(format!("start_date = '{}'", start_date));
        }

        if let Some(end_date) = &self.end_date {
            values.push(format!("end_date = '{}", end_date));
        }

        values.join(", ")
    }
}
