use std::{net::TcpListener, sync::Arc};

use actix_cors::Cors;
use actix_web::{App as ActixApp, HttpServer, dev::Server, http, middleware::Logger, web};
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
pub mod google_oauth;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod s3_client;
pub mod schema;
pub mod util;
pub mod views;

fn cors_with_allowed_origins(config: config::Server) -> Cors {
    let mut cors = Cors::default()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![
            http::header::AUTHORIZATION,
            http::header::CONTENT_TYPE,
        ])
        .supports_credentials()
        .max_age(3600);

    if config.base.production {
        let origins = config.base.allowed_origins;

        for origin in origins {
            cors = cors.allowed_origin(&origin);
        }
    } else {
        cors = cors.allow_any_origin();
    }
    cors
}

pub async fn run(listener: TcpListener, app: Arc<App>) -> Result<Server, std::io::Error> {
    let num_workers = match app.config.base.workers {
        Some(num) => num,
        None => std::thread::available_parallelism().unwrap().get(),
    };

    let state = AppState(app);

    let server = HttpServer::new(move || {
        ActixApp::new()
            .wrap(Logger::default())
            .wrap(cors_with_allowed_origins(state.config.clone()))
            .app_data(web::Data::new(state.clone()))
            .configure(routes::routes)
            .service(SwaggerUi::new("/api-docs/{_:.*}").urls(vec![(
                Url::new("API", "/api-docs/openapi.json"),
                ApiDoc::openapi(),
            )]))
    })
    .workers(num_workers)
    .listen(listener)?
    .run();

    Ok(server)
}
