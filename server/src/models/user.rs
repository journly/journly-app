use super::trip::Trip;
use crate::schema::{trips, user_trip, users};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct Collaborator {
    pub id: Uuid,
    pub username: String,
    pub avatar: Option<String>,
    pub permission: Option<String>,
}

#[derive(Clone, Debug, Queryable, Selectable, Identifiable, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub password_salt: Option<Vec<u8>>,
    pub avatar: Option<String>,
    pub provider: String,
    pub role: String,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub async fn get_all(conn: &mut AsyncPgConnection) -> QueryResult<Vec<User>> {
        users::table.select(User::as_select()).load(conn).await
    }

    pub async fn find(conn: &mut AsyncPgConnection, id: &Uuid) -> QueryResult<User> {
        users::table
            .find(id)
            .select(User::as_select())
            .first(conn)
            .await
    }

    pub async fn find_by_username(
        conn: &mut AsyncPgConnection,
        username: &str,
    ) -> QueryResult<User> {
        users::table
            .filter(users::username.eq(username))
            .first(conn)
            .await
    }

    pub async fn find_by_email(conn: &mut AsyncPgConnection, email: &str) -> QueryResult<User> {
        users::table
            .filter(users::email.eq(email))
            .first(conn)
            .await
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: &Uuid) -> QueryResult<usize> {
        diesel::delete(users::table.filter(users::id.eq(id)))
            .execute(conn)
            .await
    }

    pub async fn get_trips(conn: &mut AsyncPgConnection, user_id: &Uuid) -> QueryResult<Vec<Trip>> {
        user_trip::table
            .inner_join(trips::table)
            .filter(user_trip::user_id.eq(user_id))
            .select(Trip::as_select())
            .load(conn)
            .await
    }
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub username: Option<&'a str>,
    pub email: Option<&'a str>,
    pub password_hash: Option<&'a str>,
    pub password_salt: Option<&'a [u8]>,
    pub avatar: Option<&'a str>,
}

impl NewUser<'_> {
    pub async fn insert(&self, conn: &mut AsyncPgConnection) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(self)
            .get_result(conn)
            .await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggedUser {
    pub id: Uuid,
}

impl From<User> for LoggedUser {
    fn from(value: User) -> Self {
        Self { id: value.id }
    }
}
