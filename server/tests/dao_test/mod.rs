use journaly_server::{config::get_configuration, database::db::Database};
use uuid::Uuid;

use crate::configure_database;

#[cfg(test)]
pub mod trip_dao;

#[cfg(test)]
pub mod user_dao;

pub async fn init_db_context() -> Database {
    let config = get_configuration("test_config.toml").expect("Failed to build config");

    let mut db_config = config.db_config.clone();

    db_config.dbname = Some(Uuid::new_v4().to_string());

    configure_database(db_config.clone()).await;

    Database::new(&db_config).await
}
