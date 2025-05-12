pub mod user;

pub mod trip;

pub mod auth;

use actix_web::{HttpResponse, Responder};

const HEALTH: &str = "health";

#[utoipa::path(
    tag = HEALTH,
    get,
    path = "/health",
    responses(
        (status = 200, description = "Server is online")
    )
)]
pub async fn get_health() -> impl Responder {
    HttpResponse::Ok()
}
