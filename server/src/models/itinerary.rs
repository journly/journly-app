use super::{expenses::Expense, location::Location, trip::Trip};
use crate::schema::itinerary_items;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Queryable, Selectable, Associations)]
#[diesel(belongs_to(Trip, foreign_key = trip_id))]
#[diesel(belongs_to(Location, foreign_key = location_id))]
#[diesel(belongs_to(Expense, foreign_key = expense_id))]
pub struct ItineraryItem {
    pub id: Uuid,
    pub trip_id: Uuid,
    pub title: String,
    pub activity_type: Option<String>,
    pub location_id: Option<Uuid>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub expense_id: Option<Uuid>,
    pub notes: Option<String>,
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = itinerary_items)]
pub struct NewIntineraryItem<'a> {
    pub trip_id: &'a Uuid,
    pub title: &'a str,
}

impl NewIntineraryItem<'_> {
    pub async fn insert(&self, conn: &mut AsyncPgConnection) -> QueryResult<ItineraryItem> {
        diesel::insert_into(itinerary_items::table)
            .values(self)
            .get_result(conn)
            .await
    }
}
