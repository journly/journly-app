pub mod user_controller;

pub mod trip_controller;

pub mod auth_controller;

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
