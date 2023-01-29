use actix_web::{
    http::{header, StatusCode},
    HttpResponse, ResponseError,
};

use database::DbErr;

use derive_more::{Display, Error};

use rabbitmq_stream_client::error::ProducerPublishError;

// ----------------------------------------------------------------------

#[derive(Debug, Display, Error)]
pub enum Exception {
    Database(DbErr),
    RabbitMQ(ProducerPublishError),
}

impl ResponseError for Exception {
    fn status_code(&self) -> StatusCode {
        match *self {
            Exception::Database(DbErr::RecordNotFound(_) | DbErr::Custom(_)) => {
                StatusCode::BAD_REQUEST
            }
            Exception::Database(_) | Exception::RabbitMQ(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(header::ContentType::html())
            .body(match self {
                Exception::Database(DbErr::RecordNotFound(_) | DbErr::Custom(_)) => {
                    self.to_string()
                }
                _ => "An internal server error occurred. Please try again later.".to_owned(),
            })
    }
}
