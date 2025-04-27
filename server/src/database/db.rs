use crate::models::data_access_objects::{JoinTable, Table};
use crate::models::schema::{self, Dates};
use deadpool_postgres::{Config, Pool};
use schema::{Trip, User};
use std::{ops::DerefMut, sync::Arc};
use tokio_postgres::NoTls;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./src/migrations");
}

pub struct Database {
    pub users: Arc<Table<User>>,
    pub trips: Arc<Table<Trip>>,
    pub users_trips: Arc<JoinTable<User, Trip>>,
    pub dates: Arc<Table<Dates>>,
}

impl Database {
    pub async fn new(pg_config: Config, redis_addr: String) -> Self {
        let pg_pool = pg_config
            .create_pool(None, NoTls)
            .expect("Failed to connect to DB");

        Self::run_migrations(pg_pool.clone()).await;

        let redis_client = redis::Client::open(redis_addr).unwrap();

        let redis_pool = r2d2::Pool::builder().build(redis_client).unwrap();

        Self {
            users: Arc::from(Table::new(pg_pool.clone(), redis_pool.clone())),
            trips: Arc::from(Table::new(pg_pool.clone(), redis_pool.clone())),
            users_trips: Arc::from(JoinTable::new(pg_pool.clone(), redis_pool.clone())),
            dates: Arc::from(Table::new(pg_pool.clone(), redis_pool.clone())),
        }
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
