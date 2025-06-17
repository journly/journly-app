use crate::{auth::create_token, schema::refresh_tokens};
use bon::Builder;
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*};
use diesel_async::{
    AsyncConnection, AsyncPgConnection, RunQueryDsl, scoped_futures::ScopedFutureExt,
};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Debug, Identifiable, Queryable, Selectable, Insertable, Builder)]
#[diesel(primary_key(token))]
pub struct RefreshToken {
    pub token: String,
    pub user_id: Option<Uuid>,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub parent_token: Option<String>,
    pub revoked: bool,
}

impl RefreshToken {
    pub async fn find(conn: &mut AsyncPgConnection, token: &str) -> QueryResult<RefreshToken> {
        let refresh_token_hash = hex::encode(Sha256::digest(token.as_bytes()));

        refresh_tokens::table
            .find(refresh_token_hash)
            .select(RefreshToken::as_select())
            .first(conn)
            .await
    }

    pub async fn create(
        conn: &mut AsyncPgConnection,
        token: &str,
        user_id: &Uuid,
        expiration_in_mins: i64,
    ) -> QueryResult<()> {
        let refresh_token_hash = hex::encode(Sha256::digest(token.as_bytes()));

        let expires_at = Utc::now() + Duration::minutes(expiration_in_mins);

        let created_at = Utc::now();

        let refresh_token = RefreshToken {
            token: refresh_token_hash,
            user_id: Some(*user_id),
            expires_at: expires_at.naive_utc(),
            created_at: created_at.naive_utc(),
            parent_token: None,
            revoked: false,
        };

        insert_into(refresh_tokens::table)
            .values(&refresh_token)
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn issue_new_refresh_token(
        conn: &mut AsyncPgConnection,
        user_id: &Uuid,
        secret: &str,
        expiration_in_mins: i64,
    ) -> QueryResult<String> {
        let new_refresh_token = create_token(user_id, secret, expiration_in_mins);

        let new_refresh_token_hash = hex::encode(Sha256::digest(new_refresh_token.as_bytes()));

        let expires_at = Utc::now() + Duration::minutes(expiration_in_mins);

        let created_at = Utc::now();

        let new_record = RefreshToken {
            token: new_refresh_token_hash.to_string(),
            user_id: Some(*user_id),
            expires_at: expires_at.naive_utc(),
            created_at: created_at.naive_utc(),
            parent_token: None,
            revoked: false,
        };

        diesel::insert_into(refresh_tokens::table)
            .values(new_record)
            .execute(conn)
            .await?;

        Ok(new_refresh_token)
    }

    pub async fn issue_new(
        &self,
        conn: &mut AsyncPgConnection,
        secret: &str,
        expiration_in_mins: i64,
    ) -> QueryResult<String> {
        let new_refresh_token = create_token(&self.user_id.unwrap(), secret, expiration_in_mins);

        let new_refresh_token_hash = hex::encode(Sha256::digest(new_refresh_token.as_bytes()));

        let expires_at = Utc::now() + Duration::minutes(expiration_in_mins);

        let created_at = Utc::now();

        let new_record = RefreshToken {
            token: new_refresh_token_hash.to_string(),
            user_id: self.user_id,
            expires_at: expires_at.naive_utc(),
            created_at: created_at.naive_utc(),
            parent_token: Some(self.token.clone()),
            revoked: false,
        };

        conn.transaction(|conn| {
            async move {
                diesel::insert_into(refresh_tokens::table)
                    .values(&new_record)
                    .execute(conn)
                    .await?;

                diesel::update(refresh_tokens::table)
                    .set(refresh_tokens::revoked.eq(true))
                    .filter(refresh_tokens::token.eq(&self.token))
                    .execute(conn)
                    .await?;

                Ok(new_refresh_token)
            }
            .scope_boxed()
        })
        .await
    }

    pub async fn revoke_all_user_refresh_tokens(
        conn: &mut AsyncPgConnection,
        user_id: &Uuid,
    ) -> QueryResult<()> {
        diesel::update(refresh_tokens::table)
            .set(refresh_tokens::revoked.eq(true))
            .filter(refresh_tokens::user_id.eq(Some(user_id)))
            .filter(refresh_tokens::revoked.eq(false))
            .execute(conn)
            .await?;
        Ok(())
    }

    pub async fn revoke(&self, conn: &mut AsyncPgConnection) -> QueryResult<()> {
        diesel::update(refresh_tokens::table)
            .set(refresh_tokens::revoked.eq(true))
            .filter(refresh_tokens::token.eq(&self.token))
            .execute(conn)
            .await?;
        Ok(())
    }
}
