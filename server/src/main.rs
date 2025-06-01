use journly_server::app::App;
use journly_server::db::get_connection_pool;
use journly_server::{config::JournlyConfig, run};
use log::info;
use std::net::TcpListener;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("starting up");

    let journly_config = JournlyConfig::build();

    let db_pool = get_connection_pool(&journly_config.db).await;

    let app = Arc::new(App {
        database: db_pool,
        config: journly_config,
    });

    app.run_migrations().await;

    let listener = TcpListener::bind(format!(
        "{}:{}",
        app.config.server_addr, app.config.server_port
    ))
    .expect("Bind failed.");

    let server = run(listener, app).await?;

    server.await
}
