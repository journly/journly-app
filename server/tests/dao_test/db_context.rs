use journaly_server::{config::get_configuration, database::db::Database};

#[actix_rt::test]
async fn db_context_created() {
    let config = get_configuration("test_config.toml").expect("Failed to build config.");

    Database::new(&config.db_config).await;
}
