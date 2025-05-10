use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use typeshare::typeshare;
use utoipa::ToSchema;

use super::ToSql;

#[typeshare]
#[derive(Deserialize, Serialize, Clone, Copy, PostgresMapper, ToSchema)]
#[pg_mapper(table = "dates")]
pub struct Dates {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

impl ToSql for Dates {
    fn to_sql_values(&self) -> String {
        let mut values = Vec::new();

        if let Some(start_date) = &self.start_date {
            values.push(format!("start_date = '{}'", start_date));
        }

        if let Some(end_date) = &self.end_date {
            values.push(format!("end_date = '{}'", end_date));
        }

        values.join(", ")
    }
}
