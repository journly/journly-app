use std::sync::Arc;

use crate::{
    config::Server,
    db::{self, get_connection_pool},
    email::Emails,
    s3_client::S3Client,
    util::errors::AppError,
};
use bon::Builder;
use derive_more::Deref;
use diesel_async::{AsyncPgConnection, pooled_connection::deadpool::Pool};
use redis::Client as RedisClient;

pub type PoolResult =
    Result<diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection>, AppError>;

#[derive(Builder)]
pub struct App {
    pub database: Pool<AsyncPgConnection>,
    pub redis: RedisClient,
    pub emails: Option<Emails>,
    pub s3: Option<S3Client>,
    pub config: Server,
}

impl App {
    pub async fn from_config(config: Server) -> Self {
        let database = get_connection_pool(&config).await;

        let emails = if config.mailgun_smtp.is_some() {
            Some(Emails::from_config(&config))
        } else {
            None
        };

        let s3 = if config.s3_config.is_some() {
            Some(S3Client::from_config(&config).await)
        } else {
            None
        };

        let redis = redis::Client::open(config.redis_config.address.clone()).unwrap();

        Self {
            database,
            redis,
            emails,
            s3,
            config,
        }
    }

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
