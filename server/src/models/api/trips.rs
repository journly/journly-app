use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use utoipa::ToSchema;
use uuid::Uuid;

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
