use crate::schema::user_trip;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = user_trip)]
#[diesel(primary_key(user_id, trip_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Trip, foreign_key = trip_id))]
pub struct UserTrip {
    pub user_id: Uuid,
    pub trip_id: Uuid,
    pub permission: Option<String>,
}
