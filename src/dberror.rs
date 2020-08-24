use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::{Display, From};
use std::error::Error;
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;

pub type Result<T> = actix_web::Either<T, DataError>;

#[derive(Display, From, Debug)]
pub enum DataError {
    NotFound,
    WrongPassword,
    EmailNotFound,
    NoSessionFound,
    PGError(PGError),
    PGMError(PGMError),
    PoolError(PoolError),
}

impl std::error::Error for DataError {}

impl ResponseError for DataError {
    fn error_response(&self) -> HttpResponse {
        match self {
            DataError::NotFound => HttpResponse::NotFound().finish(),
            DataError::WrongPassword => HttpResponse::BadRequest().body("password is wrong"),
            DataError::EmailNotFound => HttpResponse::NotFound().body("Email not found"),
            DataError::NoSessionFound => HttpResponse::BadRequest().body("No session found"),
            DataError::PoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            // TODO: Don't show internal server error message to the users in prod
            DataError::PGError(err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            DataError::PGMError(err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
        }
    }
}
