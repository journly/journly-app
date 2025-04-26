use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::{Display, Error, From};
use r2d2::Error as RedisPoolError;
use tokio_pg_mapper::Error as PGMError;

#[allow(dead_code)]
#[derive(Debug, Display, Error, From)]
pub enum MyError {
    NotFound,
    InternalError,
    PGError,
    PGMError(PGMError),
    PGPoolError(PoolError),
    RedisPoolError(RedisPoolError),
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::NotFound => HttpResponse::NotFound().finish(),
            MyError::PGPoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
