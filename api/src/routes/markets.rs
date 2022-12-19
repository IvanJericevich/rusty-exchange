use crate::models::{
    error::Exception,
    market::{Market, Request},
};
use crate::AppState;

use actix_web::{get, web, HttpResponse};

use database::Query;

use utoipa::OpenApi;

// ----------------------------------------------------------------------

#[utoipa::path(
    context_path = "/markets",
    responses(
        (status = 200, description = "Returns a market with the matching base currency and quote currency", body = Market),
        (status = 500, description = "Internal server error", body = String, example = json!(String::from("An internal server error occurred. Please try again later."))),
        (status = 404, description = "Not found", body = String, example = json!(String::from("Market with symbol <base_currency>/<quote_currency> does not exist."))),
    ),
    params(
        ("base_currency", description = "Base currency of the ticker to search for"),
        ("quote_currency", description = "Quote currency of the ticker to search for")
    ),
)]
#[get("/{base_currency}/{quote_currency}")]
async fn get_by_symbol(
    path: web::Path<(String, String)>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Exception> {
    let (base_currency, quote_currency) = path.into_inner();
    let market = Query::find_market_by_ticker(&data.db, base_currency, quote_currency)
        .await
        .map_err(|e| Exception::Database(e))?;

    Ok(HttpResponse::Ok().json(market))
}

#[utoipa::path(
context_path = "/markets",
    params(Request),
    responses(
        (status = 200, description = "Returns all markets", body = [Market]),
        (status = 500, description = "Internal server error", body = String, example = json!(String::from("An internal server error occurred. Please try again later."))),
    ),
)]
#[get("")]
async fn index(
    query: web::Query<Request>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Exception> {
    let markets = Query::find_markets(&data.db, query.page.clone(), query.page_size.clone())
        .await
        .map_err(|e| Exception::Database(e))?;

    Ok(HttpResponse::Ok().json(markets))
}

// ----------------------------------------------------------------------

#[derive(OpenApi)]
#[openapi(
    paths(index, get_by_symbol),
    components(
        schemas(Market)
    ),
    tags(
        (name = "Markets", description = "Market management endpoints.")
    ),
)]
pub struct ApiDoc;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(get_by_symbol);
    cfg.service(index);
}
