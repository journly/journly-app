use crate::schema::refresh_tokens;
use chrono::NaiveDateTime;
use diesel::{
    Selectable,
    prelude::{Identifiable, Queryable},
};
use uuid::Uuid;

#[derive(Debug, Identifiable, Queryable, Selectable)]
#[diesel(primary_key(token))]
pub struct RefreshToken {
    pub token: String,
    pub user_id: Uuid,
    pub expires_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub parent_token: Option<String>,
    pub revoked: Option<bool>,
}
