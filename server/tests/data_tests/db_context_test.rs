use confik::{Configuration, EnvSource};
use dotenvy::dotenv;
use journaly_server::{config::JournalyConfig, database::db::Database};

#[actix_rt::test]
async fn db_context_created() {
    dotenv().ok();

    let config = JournalyConfig::builder()
        .override_with(EnvSource::new())
        .try_build()
        .expect("Failed to build config.");

    Database::new(config.pg).await;
}
