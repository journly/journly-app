use std::{net::TcpListener, sync::Arc};

use actix_cors::Cors;
use actix_web::{App as ActixApp, HttpServer, dev::Server, middleware::Logger, web};
use app::{App, AppState};
use routes::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

pub mod app;
pub mod auth;
pub mod config;
pub mod controllers;
pub mod db;
pub mod email;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod schema;
pub mod util;
pub mod views;

pub async fn run(listener: TcpListener, app: Arc<App>) -> Result<Server, std::io::Error> {
    let state = AppState(app);

    let server = HttpServer::new(move || {
        ActixApp::new()
            .wrap(Logger::default())
            .wrap(
            Cors::default()
                .allow_any_origin() // or .allowed_origin("http://localhost:5173")
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                .allow_any_header()
                    .max_age(3600),
            )
            .app_data(web::Data::new(state.clone()))
            .configure(|cfg| routes::routes(cfg, state.config.clone()))
            .service(SwaggerUi::new("/api-docs/{_:.*}").urls(vec![(
                Url::new("API", "/api-docs/openapi.json"),
                ApiDoc::openapi(),
            )]))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
