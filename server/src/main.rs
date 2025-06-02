use journly_server::app::App;
use journly_server::db::get_connection_pool;
use journly_server::{config::Server, run};
use log::info;
use std::net::TcpListener;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("starting up");

    let config = Server::build("config.toml");

    let db_pool = get_connection_pool(&config).await;

    let app = Arc::new(App {
        database: db_pool,
        config,
    });

    app.run_migrations().await;

    let listener = TcpListener::bind(format!(
        "{}:{}",
        app.config.server_addr, app.config.server_port
    ))
    .expect("Bind failed.");

    let server = run(listener, app).await?;

    server.await?;

    info!("Server shutting down.");

    Ok(())
}
