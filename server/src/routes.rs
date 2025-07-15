use actix_web::web::{ServiceConfig, delete, get, post, put, scope};
use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

use crate::controllers::{
    auth::{
        get_me, google_oauth, login, logout, refresh, register_user, resend_verification_code,
        verify_user_email,
    },
    get_health,
    trip::{create_trip, get_trip, get_trips},
    user::{
        change_profile_picture, delete_user, get_user, get_users, update_user, update_user_password,
    },
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::controllers::auth::register_user,
        crate::controllers::auth::get_me,
        crate::controllers::auth::login,
        crate::controllers::auth::logout,
        crate::controllers::auth::resend_verification_code,
        crate::controllers::auth::refresh,
        crate::controllers::auth::verify_user_email,
        crate::controllers::trip::get_trips,
        crate::controllers::trip::create_trip,
        crate::controllers::trip::get_trip,
        crate::controllers::user::get_users,
        crate::controllers::user::get_user,
        crate::controllers::user::delete_user,
        crate::controllers::user::update_user,
        crate::controllers::user::update_user_password,
        crate::controllers::user::change_profile_picture
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "jwt",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}

#[rustfmt::skip] // makes formatting more visually pleasing
pub fn routes(cfg: &mut ServiceConfig ) {
    cfg.route("/health", get().to(get_health))
        .service(
            scope("/api/v1/auth")
                .route("/login", post().to(login))
                .route("/logout", post().to(logout))
                .route("/refresh", post().to(refresh))
                .route("/register", post().to(register_user))
                .route("/google", get().to(google_oauth))
                .route("/me", get().to(get_me))
                .route("/resend-verification", post().to(resend_verification_code))
                .route("/verify-email", post().to(verify_user_email))
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
                .route("/{user_id}", get().to(get_user))
                .route("/{user_id}", delete().to(delete_user))
                .route("/{user_id}", put().to(update_user))
                .route("/{user_id}/password", put().to(update_user_password))
                .route("/{user_id}/profile-picture", put().to(change_profile_picture))
        );
}
