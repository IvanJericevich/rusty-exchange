use actix_web::{get, web, HttpResponse, Responder};
use utoipa::OpenApi;

#[utoipa::path(
    context_path = "/clients",
    responses(
        (status = 200, description = "List of clients", body = [String])
    )
)]
#[get("/")]
async fn get() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(OpenApi)]
#[openapi(paths(get))]
pub struct ApiDoc;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
}
