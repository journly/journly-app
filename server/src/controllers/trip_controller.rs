use actix_web::{delete, get, post, put, web::{self, Query}, HttpResponse, Responder};
use deadpool_postgres::Pool;

use crate::{ errors::MyError, models::trips::CreateTrip};

pub fn init(cfg: &mut web::ServiceConfig) {
    // cfg.service(get_trip);
    // cfg.service(create_trip);
}

// #[get("/trips")]
// pub async fn get_trip(dp_pool: web::Data<Pool>) -> impl Responder {
//     let result = dp_pool.get().await.map_err(MyError::PoolError);

//     match result {
//         Ok(client) => {
            

//             match result {
//                 Ok(trips) => HttpResponse::Ok().json(trips),
//                 Err(_) => HttpResponse::InternalServerError().into()
//             }

//         },
//         Err(_) => HttpResponse::InternalServerError().into()
//     }
// }

// #[post("/trips")]
// pub async fn create_trip(new_trip: web::<JsonCreateTrip>, dp_pool: web::Data<Pool>) -> impl Responder {
    
// }
