use crate::config::DbConfig;
use actix_web::web;
use diesel::pg::Pg;
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncConnection;
use diesel_async::AsyncPgConnection;
use diesel_migrations::embed_migrations;
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::MigrationHarness;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub type DbPool = Pool<AsyncPgConnection>;

pub enum DbError {
    MigrationFailed(&'static str),
    NotFound,
}

pub async fn get_connection_pool(db_config: DbConfig) -> DbPool {
    let url = db_config.get_db_url();
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);

    Pool::builder(manager)
        .build()
        .expect("Failed to build connection pool.")
}

pub async fn run_migration<A>(connection: A) -> Result<(), DbError>
where
    A: AsyncConnection<Backend = Pg> + 'static,
{
    let mut async_wrapper: AsyncConnectionWrapper<A> = AsyncConnectionWrapper::from(connection);

    web::block(move || {
        async_wrapper.run_pending_migrations(MIGRATIONS).unwrap();
    })
    .await
    .map_err(|_| DbError::MigrationFailed("Migration failed."))
}


