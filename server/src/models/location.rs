use crate::{schema::locations, views::EncodableLocation};
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

    pub async fn delete(conn: &mut AsyncPgConnection, id: &Uuid) -> QueryResult<usize> {
        diesel::delete(locations::table)
            .filter(locations::id.eq(id))
            .execute(conn)
            .await
    }

    pub async fn update_from_encodable(
        conn: &mut AsyncPgConnection,
        id: &Uuid,
        encodable: &EncodableLocation,
    ) -> QueryResult<usize> {
        diesel::update(locations::table)
            .filter(locations::id.eq(id))
            .set((
                locations::address.eq(&encodable.address),
                locations::display_name.eq(&encodable.display_name),
                locations::longitude.eq(encodable.longitude),
                locations::latitude.eq(encodable.latitude),
            ))
            .execute(conn)
            .await
    }
}
