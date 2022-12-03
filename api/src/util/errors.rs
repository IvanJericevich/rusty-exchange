use crate::models::client::{Client, Request};

use actix_web::{get, web, HttpResponse, Responder, ResponseError};

use actix_web::http::{header::ContentType, StatusCode};

use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum InternalError {
    #[display(fmt = "An internal server error occurred. Please try again later.")]
    DatabaseError,
}

impl ResponseError for InternalError {
    fn status_code(&self) -> StatusCode {
        match *self {
            InternalError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}
