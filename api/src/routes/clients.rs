use crate::models::{
    client::{Client, Request},
    error::Exception,
};
use crate::AppState;

use actix_web::{get, web, HttpResponse};

use database::Query;

use utoipa::OpenApi;

// ----------------------------------------------------------------------

#[utoipa::path(
    context_path = "/clients",
    responses(
        (status = 200, description = "Returns a client with the matching email address", body = Client),
        (status = 500, description = "Internal server error", body = String, example = json!(String::from("An internal server error occurred. Please try again later."))),
        (status = 404, description = "Not found", body = String, example = json!(String::from("Client with email <email> does not exist."))),
    ),
    params(
        ("email", description = "Email of the client to search for")
    ),
    tag = "Clients",
)]
#[get("/{email}")]
async fn get_by_email(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Exception> {
    let email = path.into_inner();
    let client = Query::find_client_by_email(&data.db, email.clone())
        .await
        .map_err(|e| Exception::Database(e))?;

    Ok(HttpResponse::Ok().json(client))
}

#[utoipa::path(
    context_path = "/clients",
    params(Request),
    responses(
        (status = 200, description = "Returns all clients", body = [Client]),
        (status = 500, description = "Internal server error", body = String, example = json!(String::from("An internal server error occurred. Please try again later."))),
    ),
    tag = "Clients",
)]
#[get("")]
async fn index(
    query: web::Query<Request>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Exception> {
    let clients = Query::find_clients(&data.db, query.page.clone(), query.page_size.clone())
        .await
        .map_err(|e| Exception::Database(e))?;

    Ok(HttpResponse::Ok().json(clients))
}

// ----------------------------------------------------------------------

#[derive(OpenApi)]
#[openapi(paths(index, get_by_email), components(schemas(Client)))]
pub struct ApiDoc;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(get_by_email);
    cfg.service(index);
}
