use journaly_server::init_app_state;
use journaly_server::{config::get_configuration, run};
use log::info;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("starting up");

    let config = get_configuration("config.toml").expect("Failed to read configuration");

    let app_state = init_app_state(&config).await;

    let listener = TcpListener::bind("127.0.0.1:8080").expect("Bind failed.");

    let server = run(listener, app_state).await?;

    server.await
}
