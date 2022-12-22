use crate::models::{
    error::Exception,
    order::{Order, Request},
};
use crate::AppState;

use actix_web::{get, web, HttpResponse};

use utoipa::OpenApi;

// ----------------------------------------------------------------------

#[utoipa::path(
    context_path = "/orders",
    params(Request),
    responses(
        (status = 200, description = "Returns all orders", body = [Order]),
        (status = 500, description = "Internal server error", body = String, example = json!(String::from("An internal server error occurred. Please try again later."))),
    ),
)]
#[get("")]
async fn index(
    query: web::Query<Request>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Exception> {

    Ok(HttpResponse::Ok().finish())
}

// ----------------------------------------------------------------------

#[derive(OpenApi)]
#[openapi(
    paths(index),
    components(
        schemas(Order)
    ),
    tags(
        (name = "Orders", description = "Order management endpoints.")
    ),
)]
pub struct ApiDoc;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}
