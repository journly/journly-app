use std::{
    net::TcpListener,
    sync::{Arc, Mutex},
};

use actix_web::{dev::Server, web::Data, App, HttpServer};
use config::JournalyConfig;

use crate::database::db::Database;

pub mod config;
pub mod controllers;
pub mod database;
pub mod errors;
pub mod models;

pub struct AppData {
    pub db: Arc<Database>,
    pub connections: Mutex<u32>,
}

pub fn run(listener: TcpListener, app_state: Data<AppData>) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(controllers::check_health)
            .configure(controllers::init_user_controller)
            .configure(controllers::init_trip_controller)
    })
    .listen(listener)?
    .run();

    Ok(server)
}

pub async fn init_app_state(config: &JournalyConfig) -> Data<AppData> {
    let db = Arc::new(Database::new(&config.db_config).await);

    let connections = Mutex::new(0);

    Data::new(AppData { db, connections })
}
