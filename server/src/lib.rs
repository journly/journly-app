use std::{
    net::TcpListener,
    sync::{Arc, Mutex},
};

use actix_web::{
    dev::Server,
    middleware::Logger,
    web::{Data, },
    App, HttpServer,
};
use config::JournalyConfig;
use utoipa_actix_web::AppExt;

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
            .into_utoipa_app()
            .map(|app| app.wrap(Logger::default()))
            .app_data(app_state.clone())
            .service(controllers::check_health)
            .configure(controllers::init_user_controller)
            .configure(controllers::init_trip_controller)
            .into_app()
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
