use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use utoipa_actix_web::service_config::ServiceConfig;
use uuid::Uuid;

use crate::{
    controllers::log_request,
    models::{api::{
        dates::Dates,
        trips::{CreateTrip, TripOwner, TripTitle},
    }, dao::trip_dao::TripDetails},
};
use crate::AppData;

pub fn init(cfg: &mut ServiceConfig) {
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

const TRIPS: &str = "trips";

#[utoipa::path(
    tag = TRIPS,
    responses(
        (status = 200, description = "Trips were found", body = [TripDetails])
    )
)]
#[get("/trips")]
pub async fn get_trips(app_data: web::Data<AppData>) -> impl Responder {
    log_request("GET /trips", &app_data.connections);

    let result = app_data.db.trips.get_all_trips().await;
    match result {
        Ok(trips) => HttpResponse::Ok().json(trips),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    tag = TRIPS,
    responses(
        (status = 200, description = "Trip was created", body = TripDetails)
    ) 
)]
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

#[utoipa::path(
    tag = TRIPS,
    responses(
        (status = 200, description = "Trip was found", body = TripDetails)
    )
)]
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

#[utoipa::path(
    tag = TRIPS,
    responses(
        (status = 200, description = "Trip was deleted", body = str)
    )
)]
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

#[utoipa::path(
    tag = TRIPS,
    responses(
        (status = 200, description = "Trip dates was found", body = Dates)
    )
)]
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

#[utoipa::path(
    tag = TRIPS,
    responses(
        (status = 200, description = "Trip dates was updated", body = Dates)
    )
)]
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

#[utoipa::path(
    tag = TRIPS,
    responses(
        (status = 200, description = "Trip owner was found", body = Uuid)
    )
)]
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

#[utoipa::path(
    tag = TRIPS,
    responses(
        (status = 200, description = "Trip owner was updated")
    )
)]
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

#[utoipa::path(
    tag = TRIPS,
    responses(
        (status = 200, description = "Trip title was found", body = str)
    )
)]
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

#[utoipa::path(
    tag = TRIPS,
    responses(
        (status = 200, description = "Trip title was updated", body = str)
    )
)]
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
