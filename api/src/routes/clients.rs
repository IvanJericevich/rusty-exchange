use crate::models::client::{Client, Request};

use actix_web::{get, web, HttpResponse, Responder};

use database::Query;

use utoipa::OpenApi;

use crate::AppState;

// ----------------------------------------------------------------------

#[utoipa::path(
    context_path = "/clients",
    responses(
        (status = 200, description = "Returns a client with the matching email address", body = Client),
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
        .unwrap_or_else(|_| panic!("Failed to fetch client with email address {}", email));

    HttpResponse::Ok().json(client)
}

#[utoipa::path(
    context_path = "/clients",
    params(Request),
    responses(
        (status = 200, description = "Returns all clients", body = [Client]),
    ),
)]
#[get("")]
async fn index(query: web::Query<Request>, data: web::Data<AppState>) -> impl Responder {
    let clients = Query::find_clients(&data.db, query.page, query.page_size)
        .await
        .unwrap_or_else(|_| panic!("Failed to fetch clients"));

    HttpResponse::Ok().json(clients)
}

// ----------------------------------------------------------------------

#[derive(OpenApi)]
#[openapi(
    paths(index, get_by_email),
    components(
        schemas(Client)
    ),
    tags(
        (name = "Clients", description = "Client management endpoints.")
    ),
)]
pub struct ApiDoc;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(get_by_email);
    cfg.service(index);
}
