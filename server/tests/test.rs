use std::{net::TcpListener, ops::Deref, sync::Arc};

use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use journly_server::{
    app::App,
    auth::create_access_token,
    config::{PgConfig, Server},
    db::get_connection_pool,
    email::Emails,
    run,
};
use uuid::Uuid;

pub struct TestApp {
    address: String,
    access_token: String,
}

pub async fn spawn_app() -> TestApp {
    let mut config = Server::build("test_config.toml");

    let access_token_secret = config.jwt_config.access_secret.clone();

    let db_id = configure_database(&config.postgres).await;

    config.postgres.db = db_id;

    let db_pool = get_connection_pool(&config).await;

    let emails = Emails::new_in_memory();

    let app = Arc::new(App {
        database: db_pool,
        emails,
        config,
    });

    app.run_migrations().await;

    let server_address = app.config.base.ip_address.clone();
    let server_port = app.config.base.port.clone();

    let listener =
        TcpListener::bind(format!("{}:{}", server_address, server_port)).expect("Bind failed.");

    let server = run(listener, app).await.expect("Failed to start server");

    actix_rt::spawn(server);

    TestApp {
        address: format!("http://{}:{}", server_address, server_port),
        access_token: create_access_token(Uuid::new_v4(), &access_token_secret, 10),
    }
}

pub async fn configure_database(config: &PgConfig) -> String {
    let url = config.get_db_url();

    let mut conn = AsyncPgConnection::establish(&url)
        .await
        .expect("Failed to connect to postgres database");

    let test_db_id = Uuid::new_v4();

    let query = diesel::sql_query(format!(r#"CREATE DATABASE "{}""#, test_db_id));

    query
        .execute(&mut conn)
        .await
        .unwrap_or_else(|_| panic!("Could not create database {}", test_db_id));

    test_db_id.to_string()
}

#[cfg(test)]
mod api_test;
