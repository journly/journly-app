use std::ops::DerefMut;
use deadpool_postgres::{Config, Pool};
use dotenvy::dotenv;
use tokio_postgres::NoTls;
use crate::errors::MyError;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./src/database/migrations");
}

pub async fn connection_builder(pg: Config) -> Result<Pool, MyError> {
    dotenv().ok();

    let pool = pg.create_pool(None, NoTls).expect("Failed to connect to DB");

    let mut conn = pool.get().await.unwrap();

    let client = conn.deref_mut().deref_mut();

    let report = embedded::migrations::runner().run_async(client).await.expect("Migration failed");

    println!("Migration report: {report:?}");

    Ok(pool)
}
