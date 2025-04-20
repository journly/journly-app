use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Deserialize, Serialize)]
pub struct CreateTrip {

}