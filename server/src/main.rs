use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpServer};
use config::ExampleConfig;
use confik::{Configuration, EnvSource};
use database::db::Database;
use dotenvy::dotenv;
use util::AppData;

mod config;
mod controllers;
mod database;
mod errors;
mod models;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = ExampleConfig::builder()
        .override_with(EnvSource::new())
        .try_build()
        .unwrap();

    let db = Database::new(config.pg, config.redis_addr).await;

    let app_state = web::Data::new(AppData {
        db: Arc::new(db),
        connections: Mutex::new(0),
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(controllers::init_user_controller)
            .configure(controllers::init_trip_controller)
    })
    .bind((config.server_addr.clone(), config.dev_port.clone()))?
    .run();

    println!(
        "Server running on {}:{}",
        config.server_addr, config.dev_port
    );

    server.await
}
