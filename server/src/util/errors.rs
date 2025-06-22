use std::fmt::Debug;

use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use derive_more::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum AppError {
    #[display("internal error")]
    InternalError,
    #[display("bad request: {_0}")]
    #[error(ignore)]
    BadRequest(String),
    #[display("unauthorized")]
    Unauthorized,
    NotFound,
    #[display("bad gateway")]
    BadGateway,
    #[display("forbidden")]
    Forbidden,
}

pub type AppResult<T> = Result<T, AppError>;

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::BadGateway => StatusCode::BAD_GATEWAY,
            Self::Forbidden => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
