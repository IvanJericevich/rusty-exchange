use crate::models::{sub_account::{SubAccount, Request}, error::Exception};
use crate::AppState;

use actix_web::{get, web, HttpResponse};

use database::Query;

use utoipa::OpenApi;

// ----------------------------------------------------------------------

#[utoipa::path(
    context_path = "/sub_accounts",
    responses(
        (status = 200, description = "Returns a market with the matching base currency and quote currency", body = [SubAccount]),
        (status = 500, description = "Internal server error", body = String, example = json!(String::from("An internal server error occurred. Please try again later."))),
    ),
    params(
        ("client_id", description = "Client ID for which to search sub-accounts"),
    ),
)]
#[get("/{client_id}")]
async fn get_by_client_id(path: web::Path<i32>, data: web::Data<AppState>) -> Result<HttpResponse, Exception> {
    let client_id = path.into_inner();
    let sub_accounts = Query::find_sub_accounts_by_client_id(&data.db, client_id)
        .await
        .map_err(|e| Exception::Database(e))?;

    Ok(HttpResponse::Ok().json(sub_accounts))
}

#[utoipa::path(
    context_path = "/sub_accounts",
    params(Request),
    responses(
        (status = 200, description = "Returns all markets", body = [SubAccount]),
        (status = 500, description = "Internal server error", body = String, example = json!(String::from("An internal server error occurred. Please try again later."))),
    ),
)]
#[get("")]
async fn index(query: web::Query<Request>, data: web::Data<AppState>) -> Result<HttpResponse, Exception> {
    let sub_accounts = Query::find_sub_accounts(&data.db, query.page.clone(), query.page_size.clone())
        .await
        .map_err(|e| Exception::Database(e))?;

    Ok(HttpResponse::Ok().json(sub_accounts))
}

// ----------------------------------------------------------------------

#[derive(OpenApi)]
#[openapi(
    paths(index, get_by_client_id),
    components(
        schemas(SubAccount)
    ),
    tags(
        (name = "Markets", description = "Sub-account management endpoints.")
    ),
)]
pub struct ApiDoc;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(get_by_client_id);
    cfg.service(index);
}
