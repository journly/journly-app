use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::{
    models::api::trips::{CreateTrip, UpdateTrip},
    util::AppData,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_trips);
}

#[get("/trips")]
pub async fn get_trips(app_data: web::Data<AppData>) -> impl Responder {
    let result = app_data.db.trips.get_all_trips().await;

    match result {
        Ok(trips) => HttpResponse::Ok().json(trips),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/trips")]
pub async fn create_trip(
    owner_id: web::Json<CreateTrip>,
    app_date: web::Data<AppData>,
) -> impl Responder {
    let owner_id = owner_id.into_inner().owner_id;

    let trip_result = app_date.db.trips.add_trip(owner_id).await;

    match trip_result {
        Ok(trip) => HttpResponse::Ok().json(trip),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/trips/{trip_id}")]
pub async fn get_trip(path: web::Path<Uuid>, app_date: web::Data<AppData>) -> impl Responder {
    let trip_id = path.into_inner();

    let result = app_date.db.trips.get_trip_by_id(trip_id).await;

    match result {
        Ok(trip) => HttpResponse::Ok().json(trip),
        _ => HttpResponse::InternalServerError().body("Trip could not be found."),
    }
}
//
// #[put("/trips/{trip_id}")]
// pub async fn update_trip(
//     path: web::Path<Uuid>,
//     updates: web::Json<UpdateTrip>,
//     app_date: web::Data<AppData>,
// ) -> impl Responder {
//     let trip_id = path.into_inner();
//     let updates = updates.into_inner();
//
//     if let Some(new_dates) = updates.dates {
//         let new_dates_result = app_date.db.dates.update_date_by_id(new_dates.id, new_dates);
//
//         match new_dates_result {
//             Ok(_) => {}
//         }
//     }
//
//     let result = app_date.db.trips.update_trip_by_id(trip_id, updates).await;
//
//     match result {
//         Ok(trip) => HttpResponse::Ok().json(trip),
//         _ => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// #[delete("/trips/{trip_id}")]
// pub async fn delete_trip(path: web::Path<Uuid>, app_date: web::Data<AppData>) -> impl Responder {
//     let trip_id = path.into_inner();
//
//     let result = app_date.db.trips.update_trip_by_id(trip_id, updates)
// }
