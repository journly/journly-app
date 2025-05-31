use std::future::{Ready, ready};

use crate::{
    schema::{trips, user_trip, users},
    util::errors::AppError,
};
use actix_identity::Identity;
use actix_web::{FromRequest, HttpRequest, dev::Payload};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::trip::Trip;

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
    pub password_hash: String,
    pub password_salt: Vec<u8>,
    pub avatar: Option<String>,
    pub is_admin: bool,
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

impl FromRequest for LoggedUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<LoggedUser, Self::Error>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        if let Ok(identity) = Identity::from_request(req, pl).into_inner() {
            if let Ok(user_json) = identity.id() {
                if let Ok(user) = serde_json::from_str(&user_json) {
                    return ready(Ok(user));
                }
            }
        }

        ready(Err(AppError::Unauthorized.into()))
    }
}
