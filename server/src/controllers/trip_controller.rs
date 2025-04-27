use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::{
    models::{
        api::trips::{CreateTrip, UpdateTrip},
        schema::{Dates, Trip},
    },
    util::AppData,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_trips);
}

#[get("/trips")]
pub async fn get_trips(app_data: web::Data<AppData>) -> impl Responder {
    let result = app_data.db.trips.get_trips().await;

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

    let new_dates = Dates {
        id: Uuid::new_v4(),
        start_date: None,
        end_date: None,
    };

    let dates_result = app_date.db.dates.add_dates(new_dates).await;

    match dates_result {
        Ok(dates) => {
            let new_trip = Trip {
                id: Uuid::new_v4(),
                owner_id,
                title: "".to_string(),
                trip_image: None,
                dates_id: dates.id,
            };

            let trip_result = app_date.db.trips.add_trip(new_trip).await;

            match trip_result {
                Ok(trip) => HttpResponse::Ok().json(trip),
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
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

#[put("/trips/{trip_id}")]
pub async fn update_trip(path: web::Path<Uuid>, updates: web::Json<UpdateTrip>, app_date: web::Data<AppData>) -> impl Responder {
    let trip_id = path.into_inner();
}

#[delete("/trips/{trip_id}")]
pub async fn delete_trip(path: web::Path<Uuid>, app_date: web::Data<AppData>) -> impl Responder {
    let trip_id = path.into_inner();
}
