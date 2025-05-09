use journaly_server::{config::get_configuration, run};
use journaly_server::{init_app_state, };
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Failed to read configuration");

    let app_state = init_app_state(&config).await;

    let listener = TcpListener::bind("127.0.0.1:8080").expect("Bind failed.");

    let server = run(listener, app_state)?;

    println!(
        "Server running on {}:{}",
        config.server_addr, config.dev_port
    );

    server.await
}
