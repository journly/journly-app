use actix_web::web::{ServiceConfig, delete, get, post, put, scope};
use utoipa::OpenApi;

use crate::{
    config::Server,
    controllers::{
        auth::{get_access_token, login},
        get_health,
        trip::{create_trip, get_trip, get_trips},
        user::{create_user, delete_user, get_user, get_users, update_user},
    },
};

#[derive(OpenApi)]
#[openapi(paths(
    crate::controllers::auth::get_access_token,
    crate::controllers::auth::login,
    crate::controllers::trip::get_trips,
    crate::controllers::trip::create_trip,
    crate::controllers::trip::get_trip,
    crate::controllers::user::get_users,
    crate::controllers::user::create_user,
    crate::controllers::user::get_user,
    crate::controllers::user::delete_user,
    crate::controllers::user::update_user,
))]
pub struct ApiDoc;

#[rustfmt::skip] // makes formatting more visually pleasing
pub fn routes(cfg: &mut ServiceConfig, config: Server) {
    cfg.route("/health", get().to(get_health))
        .service(
            scope("/auth")
            .route("/login", post().to(login))
        )
        .service(
            scope("/api/v1/trips")
                .route("", get().to(get_trips))
                .route("", post().to(create_trip))
                .route("/{trip_id}", get().to(get_trip))
        )
        .service(
            scope("/api/v1/users")
                .route("", get().to(get_users))
                .route("", post().to(create_user))
                .route("/{user_id}", get().to(get_user))
                .route("/{user_id}", delete().to(delete_user))
                .route("/{user_id}", put().to(update_user))
        );

    if !config.base.production {
        cfg.service(
            scope("/dev")
            .route("/access-token", get().to(get_access_token))
        );
    };
}
