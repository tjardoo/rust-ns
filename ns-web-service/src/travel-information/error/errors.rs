use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum RustNSError {
    DatabaseError(String),
    InternalServerError(String),
}

#[derive(Debug, Serialize)]
pub struct RustNSErrorResponse {
    error_message: String,
}

impl RustNSError {
    fn error_response(&self) -> String {
        match self {
            RustNSError::DatabaseError(msg) => {
                println!("Database error: {:?}", msg);

                msg.into()
            }
            RustNSError::InternalServerError(msg) => {
                println!("Internal server error: {:?}", msg);

                msg.into()
            }
        }
    }
}

impl error::ResponseError for RustNSError {
    fn status_code(&self) -> StatusCode {
        match self {
            RustNSError::DatabaseError(_msg) => StatusCode::NOT_FOUND,
            RustNSError::InternalServerError(_msg) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(RustNSErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for RustNSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for RustNSError {
    fn from(err: actix_web::error::Error) -> Self {
        RustNSError::InternalServerError(err.to_string())
    }
}

impl From<SQLxError> for RustNSError {
    fn from(err: SQLxError) -> Self {
        RustNSError::DatabaseError(err.to_string())
    }
}
