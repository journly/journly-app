use actix_web::{HttpRequest, HttpResponse, Responder, body::BoxBody, http::header::ContentType};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct OkResponse {
    #[schema(example = true)]
    pub ok: bool,
}

impl Default for OkResponse {
    fn default() -> Self {
        OkResponse::new()
    }
}

impl OkResponse {
    pub fn new() -> Self {
        Self { ok: true }
    }
}

impl Responder for OkResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
