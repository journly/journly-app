use crate::{auth::create_token, schema::refresh_tokens};
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::{prelude::*, result::Error};
use diesel_async::{
    AsyncConnection, AsyncPgConnection, RunQueryDsl, scoped_futures::ScopedFutureExt,
};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Debug, Identifiable, Queryable, Selectable, Insertable)]
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
        refresh_tokens::table
            .find(token)
            .select(RefreshToken::as_select())
            .first(conn)
            .await
    }

    pub async fn issue_new_token(
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

    pub async fn issue_new_token_from_existing(
        conn: &mut AsyncPgConnection,
        refresh_token: &str,
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

        let original_refresh_token = hex::encode(Sha256::digest(refresh_token.as_bytes()));

        conn.transaction::<(), Error, _>(|conn| {
            async move {
                diesel::insert_into(refresh_tokens::table)
                    .values(new_record)
                    .execute(conn)
                    .await?;

                diesel::update(refresh_tokens::table)
                    .set((
                        refresh_tokens::parent_token.eq(Some(new_refresh_token_hash)),
                        refresh_tokens::revoked.eq(true),
                    ))
                    .filter(refresh_tokens::token.eq(original_refresh_token))
                    .execute(conn)
                    .await?;

                Ok(())
            }
            .scope_boxed()
        })
        .await?;

        Ok(new_refresh_token)
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
