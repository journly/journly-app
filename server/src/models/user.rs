use super::trip::Trip;
use crate::{
    email::Email,
    schema::{trips, user_trip, user_verification_codes, users},
};
use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rand::Rng;
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
            .select(User::as_select())
            .filter(users::username.eq(username))
            .first(conn)
            .await
    }

    pub async fn find_by_email(conn: &mut AsyncPgConnection, email: &str) -> QueryResult<User> {
        users::table
            .select(User::as_select())
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
    pub provider: Option<&'a str>,
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

#[derive(Debug, Selectable, Queryable, Insertable)]
pub struct UserVerificationCode {
    pub email: String,
    pub verification_code: i32,
    pub expires_at: DateTime<Utc>,
}

impl UserVerificationCode {
    pub async fn generate(conn: &mut AsyncPgConnection, email: &str) -> QueryResult<i32> {
        Self::delete_existing(conn, email).await;

        let verification_code = rand::rng().random_range(100000..1000000);
        let expires_at = Utc::now() + Duration::minutes(30);

        let new_verification = UserVerificationCode {
            email: email.to_string(),
            verification_code,
            expires_at,
        };

        diesel::insert_into(user_verification_codes::table)
            .values(new_verification)
            .execute(conn)
            .await?;

        Ok(verification_code)
    }

    async fn delete_existing(conn: &mut AsyncPgConnection, email: &str) {
        let _ = diesel::delete(user_verification_codes::table)
            .filter(user_verification_codes::email.eq(email))
            .execute(conn)
            .await;
    }

    pub async fn find(
        conn: &mut AsyncPgConnection,
        email: &str,
    ) -> QueryResult<UserVerificationCode> {
        user_verification_codes::table
            .select(UserVerificationCode::as_select())
            .find(email)
            .first(conn)
            .await
    }
}

pub struct VerificationEmail<'a> {
    pub username: &'a str,
    pub verification_code: &'a i32,
}

impl Email for VerificationEmail<'_> {
    fn subject(&self) -> String {
        "Verify Your Journly Account".to_string()
    }

    fn body(&self) -> String {
        format!(
            "Hi {},\n\n\
            Welcome to Journly — we're excited to have you on board!\n\n\
            To verify your email, please enter this code in the app:\n\n\
            {}\n\n\
            This code will expire in 30 minutes.\n\n\
            If you didn’t sign up for Journly, you can ignore this message.\n\n\
            Thanks,\n\
            The Journly Team",
            self.username, self.verification_code
        )
    }
}
