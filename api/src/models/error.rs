use actix_web::{
    http::{header, StatusCode},
    HttpResponse, ResponseError,
};

use database::DbErr;

use derive_more::{Display, Error};

// ----------------------------------------------------------------------

#[derive(Debug, Display, Error)]
pub enum Exception {
    Database(DbErr),
}

impl ResponseError for Exception {
    fn status_code(&self) -> StatusCode {
        match *self {
            Exception::Database(DbErr::RecordNotFound(_)) => StatusCode::BAD_REQUEST,
            Exception::Database(DbErr::Custom(_)) => StatusCode::BAD_REQUEST,
            Exception::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(header::ContentType::html())
            .body(self.to_string())
    }
}
