use actix_web::{get, web, HttpResponse, Responder};
use utoipa::OpenApi;

#[utoipa::path(
    context_path = "/orders",
    responses(
        (status = 200, description = "List of orders", body = [String])
    )
)]
#[get("/")]
pub async fn get() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(OpenApi)]
#[openapi(paths(get))]
pub struct ApiDoc;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
}
