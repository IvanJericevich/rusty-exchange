use actix_web::{get, web, HttpResponse, Responder};

use database::Query;

use utoipa::OpenApi;

use crate::AppState;

// ----------------------------------------------------------------------

#[utoipa::path(
    context_path = "/clients",
    responses(
        (status = 200, description = "Find a client with a matching ID", body = Client),
    ),
    params(
        ("id", description = "ID of the client to search for")
    ),
)]
#[get("/{id}")]
async fn get_by_id(path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let client = Query::find_client_by_id(&data.db, id)
        .await
        .expect(format!("Client with ID {} does not exist", id).as_str());
    HttpResponse::Ok().json(client)
}

#[utoipa::path(
    context_path = "/clients",
    responses(
        (status = 200, description = "Find a client with a matching email address", body = Client),
    ),
    params(
        ("email", description = "Email of the client to search for")
    ),
)]
#[get("/{email}")]
async fn get_by_email(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let email = path.into_inner();
    let client = Query::find_client_by_email(&data.db, email.clone())
        .await
        .expect(format!("Client with email address {} does not exist", email).as_str());
    HttpResponse::Ok().json(client)
}

#[utoipa::path(
    context_path = "/clients",
    responses(
        (status = 200, description = "Get all clients", body = Client),
    ),
)]
#[get("/")]
async fn get() -> impl Responder {
    HttpResponse::Ok().body("kjsdhchbdahsc")
}

// ----------------------------------------------------------------------

#[derive(OpenApi)]
#[openapi(paths(get))]
pub struct ApiDoc;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
}
