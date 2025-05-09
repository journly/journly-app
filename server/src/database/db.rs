use crate::models::dao::Data;
use crate::models::schema::{Trip, User};
use deadpool_postgres::{Config, CreatePoolError, Pool};
use std::{ops::DerefMut, sync::Arc};
use tokio_postgres::NoTls;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!();
}

pub struct Database {
    pub users: Arc<Data<User>>,
    pub trips: Arc<Data<Trip>>,
}

impl Database {
    pub async fn new(pg_config: &Config) -> Self {
        let pg_pool = pg_config
            .create_pool(None, NoTls)
            .expect("Failed to connect to DB");

        Self::run_migrations(pg_pool.clone()).await;

        Self {
            users: Arc::from(Data::new(pg_pool.clone())),
            trips: Arc::from(Data::new(pg_pool.clone())),
        }
    }

    pub async fn create_pool(pg_config: Config) -> Result<Pool, CreatePoolError> {
        pg_config.create_pool(None, NoTls)
    }

    async fn run_migrations(pg_pool: Pool) {
        let mut conn = pg_pool.get().await.unwrap();

        let client = conn.deref_mut().deref_mut();

        match embedded::migrations::runner().run_async(client).await {
            Ok(report) => println!("Migration report: {report:?}"),
            Err(_) => println!("Migration failed."),
        }
    }
}
