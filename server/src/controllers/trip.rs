use super::helper::OkResponse;
use actix_web::web::{self, Json};
use chrono::NaiveDate;
use diesel::result::Error::NotFound;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    app::AppState,
    auth::AuthenticatedUser,
    models::trip::{NewTrip, Trip},
    util::errors::{AppError, AppResult},
    views::{EncodableTripData, EncodableTripOverview},
};

const TRIPS: &str = "trips";

#[derive(ToSchema, Serialize, Deserialize)]
pub struct GetTripsResponse {
    pub trips: Vec<EncodableTripOverview>,
}

#[utoipa::path(
    tag = TRIPS,
    get,
    path = "/api/v1/trips",
    responses(
        (status = 200, description = "Trips were found", body = GetTripsResponse)
    )
)]
pub async fn get_trips(
    authenticated: AuthenticatedUser,
    state: web::Data<AppState>,
) -> AppResult<Json<GetTripsResponse>> {
    let mut conn = state.db_connection().await?;

    match Trip::get_all(&mut conn).await {
        Ok(trips) => Ok(Json(GetTripsResponse {
            trips: trips
                .iter()
                .map(|t| EncodableTripOverview::from(t.clone()))
                .collect::<Vec<EncodableTripOverview>>(),
        })),
        Err(NotFound) => Err(AppError::BadRequest("Trip not found".to_string())),
        Err(_) => Err(AppError::InternalError),
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreateTripBody {
    pub user_id: Uuid,
    pub title: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[utoipa::path(
    tag = TRIPS,
    post,
    path = "/api/v1/trips",
    responses(
        (status = 200, description = "Trip was created", body = OkResponse)
    )
)]
pub async fn create_trip(
    authenticated: AuthenticatedUser,
    trip_data: web::Json<CreateTripBody>,
    state: web::Data<AppState>,
) -> AppResult<OkResponse> {
    let mut conn = state.db_connection().await?;

    let new_trip = NewTrip {
        owner_id: &trip_data.user_id,
        title: trip_data.title.as_deref(),
        start_date: trip_data.start_date.as_ref(),
        end_date: trip_data.end_date.as_ref(),
    };

    match new_trip.create(&mut conn).await {
        Ok(_) => Ok(OkResponse::new()),
        Err(e) => {
            eprintln!("Error during test: {}", e);
            Err(AppError::BadRequest("invalid request".to_string()))
        }
    }
}

#[derive(Deserialize, ToSchema, Serialize)]
pub struct GetTripResponse {
    pub trip: EncodableTripData,
}

#[utoipa::path(
    tag = TRIPS,
    get,
    path = "/api/v1/trips/{trip_id}",
    responses(
        (status = 200, description = "Trip was found", body = GetTripResponse),
        (status = 401, description = "User unauthorised to get trip"),
        (status = 404, description = "Trip not found")
    )
)]
pub async fn get_trip(
    authenticated: AuthenticatedUser,
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> AppResult<Json<GetTripResponse>> {
    let trip_id = path.into_inner();

    let mut conn = state.db_connection().await?;

    let user_id = authenticated.0;

    if !Trip::check_collaborator(&mut conn, &trip_id, &user_id).await {
        return Err(AppError::Unauthorized);
    }

    match Trip::get_trip_data(&mut conn, &trip_id, &user_id).await {
        Ok(data) => Ok(Json(GetTripResponse {
            trip: EncodableTripData::from(data),
        })),
        Err(NotFound) => Err(AppError::BadRequest("Trip not found".to_string())),
        Err(_) => Err(AppError::InternalError),
    }
}
