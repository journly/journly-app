use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use typeshare::typeshare;
use utoipa::ToSchema;
use uuid::Uuid;

#[typeshare]
#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "trips")]
pub struct Trip {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub title: String,
    pub image_url: Option<String>,
    pub dates_id: Uuid,
}


#[typeshare]
#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreateTrip {
    pub owner_id: Uuid,
}

#[typeshare]
#[derive(Deserialize, Serialize, ToSchema)]
pub struct TripOwner {
    pub owner_id: Uuid,
}

#[typeshare]
#[derive(Deserialize, Serialize, ToSchema)]
pub struct TripTitle {
    pub title: String,
}

#[derive(Serialize, Deserialize, PostgresMapper, ToSchema)]
#[pg_mapper(table = "trip_details")]
pub struct TripDetails {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub title: String,
    pub image_url: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}
