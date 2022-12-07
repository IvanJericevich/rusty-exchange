use crate::models::{position::{Position, Request}, error::Exception};
use crate::AppState;

use actix_web::{get, web, HttpResponse};

use database::Query;

use utoipa::OpenApi;

// ----------------------------------------------------------------------

#[utoipa::path(
    context_path = "/positions",
    params(Request),
    responses(
        (status = 200, description = "Returns all positions", body = [Position]),
        (status = 500, description = "Internal server error", body = String, example = json!(String::from("An internal server error occurred. Please try again later."))),
    ),
)]
#[get("")]
async fn index(query: web::Query<Request>, data: web::Data<AppState>) -> Result<HttpResponse, Exception> {
    let positions = Query::find_positions(
        &data.db,
        query.sub_account.clone(),
        query.base_currency.clone(),
        query.quote_currency.clone(),
        query.page.clone(),
        query.page_size.clone()
    )
        .await
        .map_err(|e| Exception::Database(e))?;

    Ok(HttpResponse::Ok().json(positions))
}

// ----------------------------------------------------------------------

#[derive(OpenApi)]
#[openapi(
    paths(index),
    components(
        schemas(Position)
    ),
    tags(
        (name = "Orders", description = "Position management endpoints.")
    ),
)]
pub struct ApiDoc;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}
