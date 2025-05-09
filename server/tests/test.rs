use std::net::TcpListener;

use deadpool_postgres::Pool;
use journaly_server::config::get_configuration;
use journaly_server::database::db::Database;
use journaly_server::init_app_state;
use tokio_postgres::NoTls;

pub async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();

    let config = get_configuration("test_config.toml").expect("Failed to build config.");

    let app_state = init_app_state(&config).await;

    let server = journaly_server::run(listener, app_state).expect("Failed to bind address");

    let _ = actix_rt::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

pub async fn init_db_context() -> Database {
    let config = get_configuration("test_config.toml").expect("Failed to build config.");

    Database::new(&config.db_config).await
}

pub async fn init_pg_pool() -> Pool {
    let config = get_configuration("test_config.toml").expect("Failed to build config.");

    let pg_config = config.db_config;

    pg_config
        .create_pool(None, NoTls)
        .expect("Failed to create Postgres pool connection.")
}

#[cfg(test)]
mod data_tests;

#[cfg(test)]
mod endpoint_tests;

#[cfg(test)]
mod config_test;
