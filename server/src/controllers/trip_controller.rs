use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::{
    controllers::log_request,
    models::api::{
        dates::Dates,
        trips::{CreateTrip, TripOwner, TripTitle},
    },
};
use crate::AppData;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_trips);
    cfg.service(create_trip);
    cfg.service(get_trip);
    cfg.service(delete_trip);
    cfg.service(get_trip_dates);
    cfg.service(update_trip_dates);
    cfg.service(get_trip_title);
    cfg.service(update_trip_title);
    cfg.service(get_trip_dates);
    cfg.service(update_trip_dates);
    cfg.service(get_trip_owner_id);
    cfg.service(update_trip_owner_id);
}

#[get("/trips")]
pub async fn get_trips(app_data: web::Data<AppData>) -> impl Responder {
    log_request("GET /trips", &app_data.connections);

    let result = app_data.db.trips.get_all_trips().await;
    match result {
        Ok(trips) => HttpResponse::Ok().json(trips),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/trips")]
pub async fn create_trip(
    owner_id: web::Json<CreateTrip>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    log_request("POST /trips", &app_data.connections);

    let owner_id = owner_id.into_inner().owner_id;

    let trip_result = app_data.db.trips.add_trip(owner_id).await;

    match trip_result {
        Ok(trip) => HttpResponse::Ok().json(trip),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/trips/{trip_id}")]
pub async fn get_trip(path: web::Path<Uuid>, app_data: web::Data<AppData>) -> impl Responder {
    let trip_id = path.into_inner();

    log_request(&format!("GET /trips/{trip_id}"), &app_data.connections);

    let result = app_data.db.trips.get_trip(trip_id).await;

    match result {
        Ok(trip) => HttpResponse::Ok().json(trip),
        _ => HttpResponse::InternalServerError().body("Trip could not be found."),
    }
}

#[delete("/trips/{trip_id}")]
pub async fn delete_trip(path: web::Path<Uuid>, app_data: web::Data<AppData>) -> impl Responder {
    let trip_id = path.into_inner();

    log_request(&format!("DELETE /trips/{trip_id}"), &app_data.connections);

    let result = app_data.db.trips.delete_trip(trip_id).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Trip was successfully deleted."),
        _ => HttpResponse::InternalServerError().into(),
    }
}

#[get("/trips/{trip_id}/dates")]
pub async fn get_trip_dates(path: web::Path<Uuid>, app_data: web::Data<AppData>) -> impl Responder {
    let trip_id = path.into_inner();

    log_request(&format!("GET /trips/{trip_id}"), &app_data.connections);

    let result = app_data.db.trips.get_trip(trip_id).await;

    match result {
        Ok(trip) => HttpResponse::Ok().json(Dates {
            start_date: trip.start_date,
            end_date: trip.end_date,
        }),
        _ => HttpResponse::InternalServerError().into(),
    }
}

#[put("/trips/{trip_id}/dates")]
pub async fn update_trip_dates(
    path: web::Path<Uuid>,
    update: web::Json<Dates>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    let trip_id = path.into_inner();

    log_request(&format!("PUT /trips/{trip_id}"), &app_data.connections);

    let new_dates = update.into_inner();

    let result = app_data
        .db
        .trips
        .update_trip_dates(trip_id, new_dates)
        .await;

    match result {
        Ok(dates) => HttpResponse::Ok().json(dates),
        _ => HttpResponse::InternalServerError().into(),
    }
}

#[get("/trips/{trip_id}/owner")]
pub async fn get_trip_owner_id(
    path: web::Path<Uuid>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    let trip_id = path.into_inner();

    log_request(
        &format!("GET /trips/{trip_id}/owner"),
        &app_data.connections,
    );

    let result = app_data.db.trips.get_trip(trip_id).await;

    match result {
        Ok(trip) => HttpResponse::Ok().json(trip.owner_id),
        _ => HttpResponse::InternalServerError().into(),
    }
}

#[put("/trips/{trip_id}/owner")]
pub async fn update_trip_owner_id(
    path: web::Path<Uuid>,
    update: web::Json<TripOwner>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    let trip_id = path.into_inner();

    log_request(
        &format!("PUT /trips/{trip_id}/owner"),
        &app_data.connections,
    );

    let new_owner_id = update.owner_id;

    let result = app_data
        .db
        .trips
        .update_trip_owner(trip_id, new_owner_id)
        .await;

    match result {
        Ok(owner_id) => HttpResponse::Ok().json(owner_id),
        _ => HttpResponse::InternalServerError().into(),
    }
}

#[get("/trips/{trip_id}/title")]
pub async fn get_trip_title(path: web::Path<Uuid>, app_data: web::Data<AppData>) -> impl Responder {
    let trip_id = path.into_inner();

    log_request(
        &format!("GET /trips/{trip_id}/title"),
        &app_data.connections,
    );

    let result = app_data.db.trips.get_trip(trip_id).await;

    match result {
        Ok(trip) => HttpResponse::Ok().json(trip.title),
        _ => HttpResponse::InternalServerError().into(),
    }
}

#[put("/trips/{trip_id}/title")]
pub async fn update_trip_title(
    path: web::Path<Uuid>,
    update: web::Json<TripTitle>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    let trip_id = path.into_inner();

    log_request(
        &format!("PUT /trips/{trip_id}/title"),
        &app_data.connections,
    );

    let new_title = update.into_inner().title;

    let result = app_data
        .db
        .trips
        .update_trip_title(trip_id, new_title)
        .await;

    match result {
        Ok(title) => HttpResponse::Ok().json(title),
        _ => HttpResponse::InternalServerError().into(),
    }
}
