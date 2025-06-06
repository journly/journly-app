use std::sync::Arc;

use bon::Builder;
use derive_more::Deref;
use diesel_async::{AsyncPgConnection, pooled_connection::deadpool::Pool};

use crate::{config::Server, db, email::Emails, util::errors::AppError};

pub type PoolResult =
    Result<diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection>, AppError>;

#[derive(Builder)]
pub struct App {
    pub database: Pool<AsyncPgConnection>,
    pub emails: Emails,
    pub config: Server,
}

impl App {
    pub async fn db_connection(&self) -> PoolResult {
        self.database
            .get()
            .await
            .map_err(|_| AppError::InternalError)
    }

    pub async fn run_migrations(&self) {
        let conn = self.database.get().await.unwrap();

        let _ = db::run_migration(conn).await;
    }
}

#[derive(Clone, Deref)]
pub struct AppState(pub Arc<App>);
