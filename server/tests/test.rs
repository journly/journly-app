use diesel_async::{
    AsyncConnection, AsyncPgConnection, RunQueryDsl, scoped_futures::ScopedFutureExt,
};
use journly_server::{
    app::App,
    auth::create_token,
    config::{PgConfig, Server},
    db::get_connection_pool,
    email::Emails,
    run,
};
use std::{net::TcpListener, sync::Arc};
use uuid::Uuid;

pub struct TestApp {
    address: String,
    access_token: String,
    database_id: String,
    config: Server,
}

pub async fn spawn_app() -> TestApp {
    let mut config = Server::build("test_config.toml");

    let test_app_config = config.clone();

    let access_token_secret = config.jwt_config.access_secret.clone();

    let db_id = configure_database(&config.postgres).await;

    config.postgres.db = db_id.clone();

    let db_pool = get_connection_pool(&config).await;

    let emails = Emails::new_in_memory();

    let app = Arc::new(App {
        database: db_pool.clone(),
        emails,
        config,
    });

    app.run_migrations().await;
    let mut conn = db_pool.clone().get().await.unwrap();

    if load_fixtures(&mut conn).await.is_err() {
        drop_database(&test_app_config.postgres.get_db_url(), &db_id).await;

        panic!("");
    };

    let listener = TcpListener::bind("127.0.0.1:0").expect("Bind failed.");
    let port = listener.local_addr().unwrap().port();

    let server = run(listener, app).await.expect("Failed to start server");

    actix_rt::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{port}"),
        access_token: create_token(
            &Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap(),
            &access_token_secret,
            10,
        ),
        database_id: db_id,
        config: test_app_config,
    }
}

async fn drop_database(db_url: &str, db_id: &str) {
    let mut conn = AsyncPgConnection::establish(db_url)
        .await
        .expect("Could not connect");

    let disconnect_users = format!(
        "SELECT pg_terminate_backend(pid)
            FROM pg_stat_activity
            WHERE datname = '{}';",
        db_id
    );

    diesel::sql_query(&disconnect_users)
        .execute(&mut conn)
        .await
        .expect("Could not disconnect");

    diesel::sql_query(format!(r#"DROP DATABASE "{}""#, db_id))
        .execute(&mut conn)
        .await
        .expect("Could not drop database");
}

impl TestApp {
    pub async fn cleanup(&self) {
        drop_database(&self.config.postgres.get_db_url(), &self.database_id).await;
    }
}

async fn configure_database(config: &PgConfig) -> String {
    let url = config.get_db_url();

    let mut conn = AsyncPgConnection::establish(&url)
        .await
        .expect("Failed to connect to postgres database");

    let test_db_id = Uuid::new_v4();

    let query = diesel::sql_query(format!(r#"CREATE DATABASE "{test_db_id}""#));

    query
        .execute(&mut conn)
        .await
        .unwrap_or_else(|_| panic!("Could not create database {test_db_id}"));

    test_db_id.to_string()
}

async fn load_fixtures(conn: &mut AsyncPgConnection) -> Result<(), diesel::result::Error> {
    let raws = vec![
        "INSERT INTO users (
            id,
            username,
            email,
            password_hash,
            password_salt,
            avatar,
            created_at,
            is_verified,
            email_verification_token,
            token_expires_at
        ) VALUES (
            '11111111-1111-1111-1111-111111111111',
            'johndoe',
            'johndoe@example.com',
            'hashed_password_123',
            decode('aabbccddeeff', 'hex'),
            'https://example.com/avatars/johndoe.png',
            '2024-01-01T12:00:00Z',
            true,
            NULL,
            NULL
        );",
        "INSERT INTO users (
            id,
            username,
            email,
            password_hash,
            password_salt,
            avatar,
            created_at,
            is_verified,
            email_verification_token,
            token_expires_at
        ) VALUES (
            '22222222-2222-2222-2222-222222222222',
            'janedoe',
            'janedoe@example.com',
            'hashed_password_456',
            decode('ffeeddccbbaa', 'hex'),
            NULL,
            '2024-02-15T09:30:00Z',
            false,
            '33333333-3333-3333-3333-333333333333',
            '2024-02-16T09:30:00Z'
        );",
        "INSERT INTO users (
            id,
            username,
            email,
            password_hash,
            password_salt,
            avatar,
            created_at,
            is_verified,
            email_verification_token,
            token_expires_at
        ) VALUES (
            '44444444-4444-4444-4444-444444444444',
            'minimaluser',
            'minimal@example.com',
            NULL,
            NULL,
            NULL,
            NULL,
            NULL,
            NULL,
            NULL
        );",
        "INSERT INTO trips (
            id,
            owner_id,
            title
        ) VALUES (
            'c8381024-3f79-4a10-b5fe-06dc24e74bdc',
            '11111111-1111-1111-1111-111111111111',
            'foo'
        );",
        "INSERT INTO user_trip (
            user_id,
            trip_id
        ) VALUES (
            '11111111-1111-1111-1111-111111111111',
            'c8381024-3f79-4a10-b5fe-06dc24e74bdc'
        );",
        "INSERT INTO budget_planners (
            trip_id
        ) VALUES (
            'c8381024-3f79-4a10-b5fe-06dc24e74bdc'
        )",
        "INSERT INTO personal_budgets (
            trip_id,
            user_id
        ) VALUES (
            'c8381024-3f79-4a10-b5fe-06dc24e74bdc',
            '11111111-1111-1111-1111-111111111111'
        )",
    ];

    conn.transaction(|conn| {
        async move {
            for raw in raws {
                diesel::sql_query(raw).execute(conn).await?;
            }

            Ok(())
        }
        .scope_boxed()
    })
    .await
}

#[cfg(test)]
mod api_test;
