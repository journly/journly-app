use actix_web::{ web, App, HttpServer };
use confik::{ Configuration, EnvSource };
use database::{config::ExampleConfig, db::connection_builder};
use dotenvy::dotenv;

mod models;
mod errors;
mod database;
mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = ExampleConfig::builder()
        .override_with(EnvSource::new())
        .try_build()
        .unwrap();

    let pool = connection_builder(config.pg).await.unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(controllers::init_user_controller)
            .configure(controllers::init_trip_controller)
            
    })
        .bind((config.server_addr.clone(), config.dev_port.clone()))?
        .run();

    println!("Server running on {}:{}", config.server_addr, config.dev_port);

    server.await
}
