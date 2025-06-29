use journly_server::app::App;
use journly_server::db::get_connection_pool;
use journly_server::email::Emails;
use journly_server::s3_client::S3Client;
use journly_server::{config::Server, run};
use log::info;
use std::net::TcpListener;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("starting up");

    let config = Server::build("config.toml");

    let database = get_connection_pool(&config).await;

    let emails = Emails::from_config(&config);

    let s3_config = S3Client::build_config(&config.s3_config).await;

    let s3 = S3Client::new(
        &s3_config,
        &config.s3_config.bucket_name,
        &config.s3_config.base_url,
    );

    let app = Arc::new(App {
        database,
        emails,
        config,
        s3,
    });

    app.run_migrations().await;

    let listener = TcpListener::bind(format!(
        "{}:{}",
        app.config.base.ip_address, app.config.base.port
    ))
    .expect("Bind failed.");

    let server = run(listener, app).await?;

    server.await?;

    info!("Server shutting down.");

    Ok(())
}
