pub mod user_controller;

pub mod trip_controller;

use std::sync::Mutex;

use actix_web::{get, HttpResponse, Responder};
pub use user_controller::init as init_user_controller;

pub use trip_controller::init as init_trip_controller;

pub fn log_request(endpoint: &str, connection: &Mutex<u32>) {
    let mut num = connection.lock().unwrap();
    *num += 1;
    println!("Connection {}, requested: '{}'", *num, endpoint);
}

#[get("/health")]
pub async fn check_health() -> impl Responder {
    HttpResponse::Ok()
}
