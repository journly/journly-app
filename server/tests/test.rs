use std::{net::TcpListener, sync::Arc};
use tokio::runtime::Runtime;
use diesel_async::{pooled_connection::deadpool::Pool, AsyncConnection, AsyncPgConnection, RunQueryDsl};
use journly_server::{
    app::App, auth::create_access_token, config::{PgConfig, Server}, db::get_connection_pool, email::Emails, models::user::NewUser, run, schema::users
};
use uuid::Uuid;

pub struct TestApp {
    address: String,
    access_token: String,
    database_id: String,
    database: Pool<AsyncPgConnection>,
}

pub async fn spawn_app() -> TestApp {
    let mut config = Server::build("test_config.toml");

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
    load_fixtures(&mut conn).await.unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").expect("Bind failed.");
    let port = listener.local_addr().unwrap().port();

    let server = run(listener, app).await.expect("Failed to start server");

    actix_rt::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        access_token: create_access_token(Uuid::new_v4(), &access_token_secret, 10),
        database_id: db_id,
        database: db_pool,
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        // Use the current runtime's handle to spawn the cleanup task
        let database = self.database.clone();
        let database_id = self.database_id.clone();
        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            handle.spawn(async move {
                let mut conn = database.get().await.unwrap();
                diesel::sql_query(format!(r#"DELETE DATABASE "{}""#, database_id))
                    .execute(&mut conn)
                    .await
                    .unwrap();
            });
        } else {
            // Fallback for non-tokio contexts (not recommended for production)
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async {
                let mut conn = self.database.get().await.unwrap();
                diesel::sql_query(format!(r#"DELETE DATABASE "{}""#, self.database_id))
                    .execute(&mut conn)
                    .await
                    .unwrap();
            });
        }
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
            '11111111-1111-1111-1111-111111111111'::UUID,
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
            '22222222-2222-2222-2222-222222222222'::UUID,
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
            '44444444-4444-4444-4444-444444444444'::UUID,
            'minimaluser',
            'minimal@example.com',
            NULL,
            NULL,
            NULL,
            NULL,
            NULL,
            NULL,
            NULL
        );"
    ];
    for raw in raws {
        diesel::sql_query(raw)
            .execute(conn)
            .await?;
    }
    Ok(())
}



#[cfg(test)]
mod api_test;
