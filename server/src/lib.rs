use std::{net::TcpListener, sync::Arc};

use actix_web::{dev::Server, middleware::Logger, web::Data, App, HttpServer};
use config::JournalyConfig;
use routes::{TripsApiDoc, UsersApiDoc};
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

use crate::database::db::Database;

pub mod config;
pub mod controllers;
pub mod database;
pub mod errors;
pub mod models;
pub mod routes;

pub struct AppData {
    pub db: Arc<Database>,
}

pub fn run(listener: TcpListener, app_state: Data<AppData>) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .configure(routes::routes)
            .service(SwaggerUi::new("/api-docs/{_:.*}").urls(vec![
                (
                    Url::new("Trips API", "/api-docs/trips-openapi.json"),
                    TripsApiDoc::openapi(),
                ),
                (
                    Url::with_primary("Users API", "/api-docs/users-openapi.json", true),
                    UsersApiDoc::openapi(),
                ),
            ]))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

pub async fn init_app_state(config: &JournalyConfig) -> Data<AppData> {
    let db = Arc::new(Database::new(&config.db_config).await);

    Data::new(AppData { db })
}
