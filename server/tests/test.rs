use std::net::TcpListener;

use deadpool_postgres::Pool;
use journaly_server::config::get_configuration;
use journaly_server::database::db::Database;
use journaly_server::init_app_state;
use tokio_postgres::NoTls;
use uuid::Uuid;

pub async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();

    let mut config = get_configuration("test_config.toml").expect("Failed to build config.");

    config.db_config.dbname = Some(Uuid::new_v4().to_string());

    configure_database(config.db_config.clone()).await;

    let app_state = init_app_state(&config).await;

    let server = journaly_server::run(listener, app_state).expect("Failed to bind address");

    actix_rt::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

pub async fn configure_database(config: deadpool_postgres::Config) {
    let mut postgres_config = tokio_postgres::Config::new();
    postgres_config.host(config.host.unwrap());
    postgres_config.dbname("postgres");
    postgres_config.port(config.port.unwrap());
    postgres_config.user(config.user.unwrap());
    postgres_config.password(config.password.unwrap());

    let (client, connection) = postgres_config
        .connect(NoTls)
        .await
        .expect("Failed to connect to Postgres");

    actix_rt::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let _ = client
        .execute(
            &format!(r#"CREATE DATABASE "{}""#, config.dbname.unwrap()),
            &[],
        )
        .await;
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
