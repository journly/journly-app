use std::sync::Arc;

use bon::Builder;
use derive_more::Deref;
use diesel_async::{AsyncPgConnection, pooled_connection::deadpool::Pool};

use crate::{config, util::errors::AppError};

pub type PoolResult =
    Result<diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection>, AppError>;

#[derive(Builder)]
pub struct App {
    pub database: Pool<AsyncPgConnection>,
    pub config: config::Server,
}

impl App {
    pub async fn db_connection(&self) -> PoolResult {
        self.database
            .get()
            .await
            .map_err(|_| AppError::InternalError)
    }
}

#[derive(Clone, Deref)]
pub struct AppState(pub Arc<App>);
