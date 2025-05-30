use super::trip::Trip;
use crate::schema::itinerary_items;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

#[derive(Debug, Identifiable, Queryable, Selectable, Associations)]
#[diesel(belongs_to(Trip, foreign_key = trip_id))]
#[diesel(table_name = itinerary_items)]
pub struct ItineraryItem {
    id: Uuid,
    trip_id: Uuid,
    title: String,
    activity_type: Option<String>,
    location_id: Option<Uuid>,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    expense_id: Option<Uuid>,
    notes: Option<String>,
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = itinerary_items)]
pub struct NewIntineraryItem<'a> {
    trip_id: &'a Uuid,
    title: &'a str,
}

impl NewIntineraryItem<'_> {
    pub async fn insert(&self, conn: &mut AsyncPgConnection) -> QueryResult<ItineraryItem> {
        diesel::insert_into(itinerary_items::table)
            .values(self)
            .get_result(conn)
            .await
    }
}
