use std::fmt::Debug;

use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Display, Error)]
pub enum AppError {
    #[display("internal_error")]
    InternalError,
    #[display("bad_request")]
    #[error(ignore)]
    BadRequest(&'static str),
    #[display("unauthorized")]
    #[error(ignore)]
    Unauthorized(&'static str),
    NotFound,
    #[display("bad_gateway")]
    BadGateway,
    #[display("forbidden")]
    #[error(ignore)]
    Forbidden(&'static str),
    #[display("conflict")]
    Conflict,
    #[display("unverified_user")]
    #[error(ignore)]
    UnverifiedUser(&'static str),
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    #[schema(example = "internal_error")]
    error: String,
    #[schema(example = "Internal server error.")]
    message: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::BadGateway => StatusCode::BAD_GATEWAY,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::Conflict => StatusCode::CONFLICT,
            Self::UnverifiedUser(_) => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            Self::InternalError => HttpResponse::build(self.status_code()).json(ErrorResponse {
                error: self.to_string(),
                message: "Internal server error.".to_string(),
            }),
            Self::BadRequest(msg) => HttpResponse::build(self.status_code()).json(ErrorResponse {
                error: self.to_string(),
                message: msg.to_string(),
            }),
            Self::Unauthorized(msg) => {
                HttpResponse::build(self.status_code()).json(ErrorResponse {
                    error: self.to_string(),
                    message: msg.to_string(),
                })
            }
            Self::NotFound => HttpResponse::build(self.status_code()).json(ErrorResponse {
                error: self.to_string(),
                message: "Resource not found.".to_string(),
            }),
            Self::BadGateway => HttpResponse::build(self.status_code()).json(ErrorResponse {
                error: self.to_string(),
                message: "Bad gateway.".to_string(),
            }),
            Self::Forbidden(msg) => HttpResponse::build(self.status_code()).json(ErrorResponse {
                error: self.to_string(),
                message: msg.to_string(),
            }),
            Self::Conflict => HttpResponse::build(self.status_code()).json(ErrorResponse {
                error: self.to_string(),
                message: "Conflict occurred.".to_string(),
            }),
            Self::UnverifiedUser(msg) => {
                HttpResponse::build(self.status_code()).json(ErrorResponse {
                    error: self.to_string(),
                    message: msg.to_string(),
                })
            }
        }
    }
}
