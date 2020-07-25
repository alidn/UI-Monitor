use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::{Display, From};
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;

pub type Result<T> = actix_web::Either<T, DataError>;

#[derive(Display, From, Debug)]
pub enum DataError {
    NotFound,
    WrongPassword,
    EmailNotFound,
    PGError(PGError),
    PGMError(PGMError),
    PoolError(PoolError),
}

impl std::error::Error for DataError {}

impl ResponseError for DataError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            DataError::NotFound => HttpResponse::NotFound().finish(),
            DataError::WrongPassword => HttpResponse::BadRequest().body("password is wrong"),
            DataError::EmailNotFound => HttpResponse::NotFound().body("Email not found"),
            DataError::PoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
