use actix_session::Session;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct LoginCredentials {
    username: Option<String>,
    email: Option<String>,
    password: String,
}

#[utoipa::path(
    tag = "test",
    post,
    path = "/api/auth/login",
    responses(
        (status = 200, description = "Login was successful", body = str)
    ),
    params(
        ("id" = u64, Path, description = "id to test shit"),
    )
)]
pub async fn login(credentials: LoginCredentials, session: Session) -> impl Responder {
    if credentials.username.is_none() && credentials.email.is_none() {
        return HttpResponse::BadRequest().body("Missing username or email.");
    }
    HttpResponse::Ok().into()
}
