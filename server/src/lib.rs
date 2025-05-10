use std::{
    net::TcpListener,
    sync::{Arc},
};

use actix_web::{dev::Server, middleware::Logger, web::Data, App, HttpServer};
use config::JournalyConfig;
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

use crate::database::db::Database;

pub mod config;
pub mod controllers;
pub mod database;
pub mod errors;
pub mod models;

pub struct AppData {
    pub db: Arc<Database>,
}

pub fn run(listener: TcpListener, app_state: Data<AppData>) -> Result<Server, std::io::Error> {
    #[derive(OpenApi)]
    #[openapi()]
    struct ApiDoc;

    let server = HttpServer::new(move || {
        App::new()
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .map(|app| app.wrap(Logger::default()))
            .app_data(app_state.clone())
            .service(controllers::check_health)
            .configure(controllers::init_user_controller)
            .configure(controllers::init_trip_controller)
            .openapi_service(|api| {
                SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .into_app()
    })
    .listen(listener)?
    .run();

    Ok(server)
}

pub async fn init_app_state(config: &JournalyConfig) -> Data<AppData> {
    let db = Arc::new(Database::new(&config.db_config).await);

    Data::new(AppData { db })
}
