use std::{net::TcpListener, sync::Arc};

use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use journly_server::{
    app::App,
    config::{DbConfig, JournlyConfig},
    db::get_connection_pool,
    run,
};
use uuid::Uuid;

pub async fn spawn_app() -> String {
    let mut journly_config = JournlyConfig::build("test_config.toml");

    let db_id = configure_database(&journly_config.db_config).await;

    journly_config.db_config.pg_db = db_id;

    let db_pool = get_connection_pool(&journly_config.db_config).await;

    let app = Arc::new(App {
        database: db_pool,
        config: journly_config,
    });

    app.run_migrations().await;

    let server_address = app.config.server_addr.clone();
    let server_port = app.config.server_port.clone();

    let listener =
        TcpListener::bind(format!("{}:{}", server_address, server_port)).expect("Bind failed.");

    let server = run(listener, app).await.expect("Failed to start server");

    actix_rt::spawn(server);

    format!("http://{}:{}", server_address, server_port)
}

pub async fn configure_database(config: &DbConfig) -> String {
    let url = config.get_db_url();

    let mut conn = AsyncPgConnection::establish(&url)
        .await
        .expect("Failed to connect to postgres database");

    let test_db_id = Uuid::new_v4();

    let query = diesel::sql_query(format!("CREATE DATABASE {}", test_db_id));

    query
        .execute(&mut conn)
        .await
        .expect(&format!("Could not create database {}", test_db_id));

    test_db_id.to_string()
}

#[cfg(test)]
mod api_test;

#[cfg(test)]
mod config_test;
