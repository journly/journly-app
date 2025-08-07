// use actix_web::web;
// use serde::Deserialize;
// use utoipa::ToSchema;
// use uuid::Uuid;
//
// use crate::{
//     app::AppState,
//     auth::AuthenticatedUser,
//     util::errors::{AppError, AppResult},
//     views::EncodableItineraryItem,
// };
//
// use super::helper::OkResponse;
//
// const TRIPS: &str = "trips";
//
// #[derive(Deserialize, ToSchema)]
// pub struct UpdateItineraryBody {
//     r#type: String,
//     item_id: Option<Uuid>,
//     item: Option<EncodableItineraryItem>,
// }
//
// #[utoipa::path(
//     tag = TRIPS,
//     patch,
//     path = "/api/v1/trips/{trip_id}/itinerary",
//     responses (
//         (status = 200, description = "Trip itinerary successfully updated", body = inline(OkResponse))
//     ),
//     security(
//         ("jwt" = [])
//     )
//
// )]
// pub async fn update_itinerary(
//     authenticated: AuthenticatedUser,
//     path: web::Path<Uuid>,
//     state: web::Data<AppState>,
//     body: web::Json<UpdateItineraryBody>,
// ) -> AppResult<OkResponse> {
//     match body.r#type.as_str() {
//         "add_item" => {
//             let new_item = match body.item {
//                 Some(item) => Ok(item),
//                 None => Err(AppError::BadRequest("Missing itinerary item.".to_string())),
//             }?;
//         }
//         "remove_item" => {}
//         "update_item" => {}
//         _ => Err(AppError::BadRequest(
//             "Invalid type. Use one of the valid types 'add_item', 'remove_item', 'update_item'"
//                 .to_string(),
//         )),
//     }
// }
