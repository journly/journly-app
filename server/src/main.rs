use journaly_server::config::DbConfig;
use journaly_server::database::db::get_connection_pool;
use journaly_server::init_app_state;
use journaly_server::{config::get_configuration, run};
use log::info;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("starting up");

    let db_config = DbConfig::get_config();

    let db_pool = get_connection_pool(db_config);

    let listener = TcpListener::bind("127.0.0.1:8080").expect("Bind failed.");

    let server = run(listener, db_pool).await?;

    server.await
}
