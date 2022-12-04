use actix_web::{HttpResponse, ResponseError, http::{header, StatusCode}};

use database::DbErr;

use derive_more::{Display, Error};

// ----------------------------------------------------------------------

#[derive(Debug, Display, Error)]
pub enum Exception {
    #[display(fmt = "An internal server error occurred. Please try again later.")]
    Database(DbErr),
}

impl ResponseError for Exception {
    fn status_code(&self) -> StatusCode {
        match *self {
            Exception::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(header::ContentType::html())
            .body(self.to_string())
    }
}
