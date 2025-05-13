use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::{
    models::api::{
        dates::Dates,
        trips::{CreateTrip, TripDetails, TripOwner, TripTitle},
    },
    AppData,
};

const TRIPS: &str = "trips";

#[utoipa::path(
    tag = TRIPS,
    get,
    path = "/api/trips",
    responses(
        (status = 200, description = "Trips were found", body = [TripDetails])
    )
)]
pub async fn get_trips(app_data: web::Data<AppData>) -> impl Responder {
    let result = app_data.db.trips.get_all_trips().await;

    match result {
        Ok(trips) => HttpResponse::Ok().json(trips),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    tag = TRIPS,
    post,
    path = "/api/trips",
    responses(
        (status = 200, description = "Trip was created", body = TripDetails)
    )
)]
pub async fn create_trip(
    owner_id: web::Json<CreateTrip>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    let owner_id = owner_id.into_inner().owner_id;

    let trip_result = app_data.db.trips.add_trip(owner_id).await;

    match trip_result {
        Ok(trip) => HttpResponse::Ok().json(trip),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    tag = TRIPS,
    get,
    path = "/api/trips/{trip_id}",
    responses(
        (status = 200, description = "Trip was found", body = TripDetails)
    )
)]
pub async fn get_trip(path: web::Path<Uuid>, app_data: web::Data<AppData>) -> impl Responder {
    let trip_id = path.into_inner();

    let result = app_data.db.trips.get_trip(trip_id).await;

    match result {
        Ok(trip) => HttpResponse::Ok().json(trip),
        Err(_) => HttpResponse::InternalServerError().body("Trip could not be found."),
    }
}

#[utoipa::path(
    tag = TRIPS,
    delete,
    path = "/api/trips/{trip_id}",
    responses(
        (status = 200, description = "Trip was deleted", body = str)
    )
)]
pub async fn delete_trip(path: web::Path<Uuid>, app_data: web::Data<AppData>) -> impl Responder {
    let trip_id = path.into_inner();

    let result = app_data.db.trips.delete_trip(trip_id).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Trip was successfully deleted."),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[utoipa::path(
    tag = TRIPS,
    get,
    path = "/api/trips/{trip_id}/dates",
    responses(
        (status = 200, description = "Trip dates was found", body = Dates)
    )
)]
pub async fn get_trip_dates(path: web::Path<Uuid>, app_data: web::Data<AppData>) -> impl Responder {
    let trip_id = path.into_inner();

    let result = app_data.db.trips.get_trip(trip_id).await;

    match result {
        Ok(trip) => HttpResponse::Ok().json(Dates {
            start_date: trip.start_date,
            end_date: trip.end_date,
        }),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[utoipa::path(
    tag = TRIPS,
    put,
    path = "/api/trips/{trip_id}/dates",
    responses(
        (status = 200, description = "Trip dates was updated", body = Dates)
    )
)]
pub async fn update_trip_dates(
    path: web::Path<Uuid>,
    update: web::Json<Dates>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    let trip_id = path.into_inner();

    let new_dates = update.into_inner();

    let result = app_data
        .db
        .trips
        .update_trip_dates(trip_id, new_dates)
        .await;

    match result {
        Ok(dates) => HttpResponse::Ok().json(dates),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[utoipa::path(
    tag = TRIPS,
    get,
    path = "/api/trips/{trip_id}/owner",
    responses(
        (status = 200, description = "Trip owner was found", body = Uuid)
    )
)]
pub async fn get_trip_owner_id(
    path: web::Path<Uuid>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    let trip_id = path.into_inner();

    let result = app_data.db.trips.get_trip(trip_id).await;

    match result {
        Ok(trip) => HttpResponse::Ok().json(trip.owner_id),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[utoipa::path(
    tag = TRIPS,
    put,
    path = "/api/trips/{trip_id}/owner",
    responses(
        (status = 200, description = "Trip owner was updated")
    )
)]
pub async fn update_trip_owner_id(
    path: web::Path<Uuid>,
    update: web::Json<TripOwner>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    let trip_id = path.into_inner();

    let new_owner_id = update.owner_id;

    let result = app_data
        .db
        .trips
        .update_trip_owner(trip_id, new_owner_id)
        .await;

    match result {
        Ok(owner_id) => HttpResponse::Ok().json(owner_id),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[utoipa::path(
    tag = TRIPS,
    get,
    path = "/api/trips/{trip_id}/title",
    responses(
        (status = 200, description = "Trip title was found", body = str)
    )
)]
pub async fn get_trip_title(path: web::Path<Uuid>, app_data: web::Data<AppData>) -> impl Responder {
    let trip_id = path.into_inner();

    let result = app_data.db.trips.get_trip(trip_id).await;

    match result {
        Ok(trip) => HttpResponse::Ok().json(trip.title),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[utoipa::path(
    tag = TRIPS,
    put,
    path = "/api/trips/{trip_id}/title",
    responses(
        (status = 200, description = "Trip title was updated", body = str)
    )
)]
pub async fn update_trip_title(
    path: web::Path<Uuid>,
    update: web::Json<TripTitle>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    let trip_id = path.into_inner();

    let new_title = update.into_inner().title;

    let result = app_data
        .db
        .trips
        .update_trip_title(trip_id, new_title)
        .await;

    match result {
        Ok(title) => HttpResponse::Ok().json(title),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
