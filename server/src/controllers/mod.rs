pub mod user_controller;

pub mod trip_controller;

use actix_web::{get, HttpResponse, Responder};

pub use user_controller::init as init_user_controller;

pub use trip_controller::init as init_trip_controller;

const HEALTH: &str = "health";

#[utoipa::path(
    tag = HEALTH,
    responses(
        (status = 200, description = "Server is online")
    )
)]
#[get("/health")]
pub async fn check_health() -> impl Responder {
    HttpResponse::Ok()
}
