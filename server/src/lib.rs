use std::{net::TcpListener, sync::Arc};

use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, storage::RedisSessionStore};
use actix_web::{App as ActixApp, HttpServer, dev::Server, middleware::Logger, web};
use app::{App, AppState};
use routes::{AuthApiDoc, TripsApiDoc, UsersApiDoc};
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

pub mod app;
pub mod config;
pub mod controllers;
pub mod db;
pub mod models;
pub mod routes;
pub mod schema;
pub mod util;
pub mod views;

pub async fn run(listener: TcpListener, app: Arc<App>) -> Result<Server, std::io::Error> {
    let state = AppState(app);

    let secret_key = actix_web::cookie::Key::generate();

    let store = RedisSessionStore::new(state.config.redis.get_redis_url())
        .await
        .unwrap();

    let server = HttpServer::new(move || {
        ActixApp::new()
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::builder(store.clone(), secret_key.clone()).build())
            .wrap(Logger::default())
            .app_data(web::Data::new(state.clone()))
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
                (
                    Url::new("Auth API", "/api-docs/auth-openapi.json"),
                    AuthApiDoc::openapi(),
                ),
            ]))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
