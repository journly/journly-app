use super::trip::Trip;
use crate::schema::documents;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, Queryable, Selectable, Associations)]
#[diesel(belongs_to(Trip, foreign_key = trip_id))]
pub struct Document {
    pub id: Uuid,
    pub trip_id: Uuid,
    pub filename: String,
    pub document_url: String,
    pub file_hash: String,
    pub file_type: String,
    pub file_size: i64,
    pub created_at: DateTime<Utc>,
}
