use crate::schema::locations;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Queryable, Selectable)]
pub struct Location {
    pub id: Uuid,
    pub address: String,
    pub display_name: Option<String>,
    pub longitude: f64,
    pub latitude: f64,
}

impl Location {
    pub async fn find(conn: &mut AsyncPgConnection, id: &Uuid) -> QueryResult<Location> {
        locations::table
            .find(id)
            .select(Location::as_select())
            .first(conn)
            .await
    }
}
