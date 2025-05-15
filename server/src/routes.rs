use actix_web::web::{delete, get, post, put, scope, ServiceConfig};
use utoipa::OpenApi;

use crate::handlers::{
    get_health,
    trip::{
        create_trip, delete_trip, get_trip, get_trip_dates, get_trip_owner_id, get_trip_title,
        get_trips, update_trip_dates, update_trip_owner_id, update_trip_title,
    },
    user::{
        create_user, delete_user, get_user, get_user_display_name, get_user_email, get_users,
        update_user_display_name, update_user_email,
    },
};

#[derive(OpenApi)]
#[openapi(paths(
    crate::handlers::trip::get_trips,
    crate::handlers::trip::create_trip,
    crate::handlers::trip::get_trip,
    crate::handlers::trip::delete_trip,
    crate::handlers::trip::get_trip_dates,
    crate::handlers::trip::update_trip_dates,
    crate::handlers::trip::get_trip_owner_id,
    crate::handlers::trip::update_trip_owner_id,
    crate::handlers::trip::get_trip_title,
    crate::handlers::trip::update_trip_title,
))]
pub struct TripsApiDoc;

#[derive(OpenApi)]
#[openapi(paths(
    crate::handlers::user::get_users,
    crate::handlers::user::create_user,
    crate::handlers::user::get_user,
    crate::handlers::user::delete_user,
    crate::handlers::user::get_user_display_name,
    crate::handlers::user::update_user_display_name,
    crate::handlers::user::get_user_email,
    crate::handlers::user::update_user_email
))]
pub struct UsersApiDoc;

#[rustfmt::skip] // makes formatting more visually pleasing
pub fn routes(cfg: &mut ServiceConfig) {
    cfg.route("/health", get().to(get_health))
        .service(
            scope("/api/trips")
                .route("", get().to(get_trips))
                .route("", post().to(create_trip))
                .route("/{trip_id}", get().to(get_trip))
                .route("/{trip_id}", delete().to(delete_trip))
                .route("/{trip_id}/dates", get().to(get_trip_dates))
                .route("/{trip_id}/dates", put().to(update_trip_dates))
                .route("/{trip_id}/owner", get().to(get_trip_owner_id))
                .route("/{trip_id}/owner", put().to(update_trip_owner_id))
                .route("/{trip_id}/title", get().to(get_trip_title))
                .route("/{trip_id}/title", put().to(update_trip_title)),
        )
        .service(
            scope("/api/users")
                .route("", get().to(get_users))
                .route("", post().to(create_user))
                .route("/{user_id}", get().to(get_user))
                .route("/{user_id}", delete().to(delete_user))
                .route("/{user_id}/display_name", get().to(get_user_display_name))
                .route("/{user_id}/display_name", put().to(update_user_display_name))
                .route("/{user_id}/email", get().to(get_user_email))
                .route("/{user_id}/email", put().to(update_user_email)),
        );
}
