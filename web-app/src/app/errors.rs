use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum WebAppError {
    InternalServerError(String),
}

#[derive(Debug, Serialize)]
pub struct WebAppErrorResponse {
    error_message: String,
}

impl WebAppError {
    fn error_response(&self) -> String {
        match self {
            WebAppError::InternalServerError(msg) => {
                println!("Internal server error: {:?}", msg);

                msg.into()
            }
        }
    }
}

impl error::ResponseError for WebAppError {
    fn status_code(&self) -> StatusCode {
        match self {
            WebAppError::InternalServerError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(WebAppErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for WebAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for WebAppError {
    fn from(err: actix_web::error::Error) -> Self {
        WebAppError::InternalServerError(err.to_string())
    }
}
