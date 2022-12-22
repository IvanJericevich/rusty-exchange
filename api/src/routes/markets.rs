use crate::models::{
    error::Exception,
    market::{Market, GetRequest, PostRequest, PutRequest},
};
use crate::AppState;

use actix_web::{get, post, put, web, HttpResponse};

use database::{Mutation, Query};

use utoipa::OpenApi;

// ----------------------------------------------------------------------

#[utoipa::path(
    context_path = "/markets",
    params(GetRequest),
    responses(
        (status = 200, description = "Returns all markets", body = [Market]),
        (status = 500, description = "Internal server error", body = String, example = json!("An internal server error occurred. Please try again later.")),
    ),
)]
#[get("/")]
async fn get(
    query: web::Query<GetRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Exception> {
    let markets = Query::find_markets(&data.db, query.page.clone(), query.page_size.clone())
        .await
        .map_err(|e| Exception::Database(e))?;

    Ok(HttpResponse::Ok().json(markets))
}

#[utoipa::path(
    context_path = "/markets",
    responses(
        (status = 200, description = "Returns a market with the matching base currency and quote currency", body = Market),
        (status = 500, description = "Internal server error", body = String, example = json!("An internal server error occurred. Please try again later.")),
        (status = 400, description = "Bad request", body = String, example = json!("Market with symbol <base_currency>/<quote_currency> does not exist.")),
    ),
    params(
        ("base_currency", description = "Base currency of the ticker to search for."),
        ("quote_currency", description = "Quote currency of the ticker to search for.")
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
    params(
        ("base_currency", description = "New base currency of the market to create."),
        ("quote_currency", description = "New quote currency of the market to create.")
    ),
    request_body = PostRequest,
    responses(
        (status = 200, description = "Returns the created market record", body = Client),
        (status = 500, description = "Internal server error", body = String, example = json!("An internal server error occurred. Please try again later.")),
        (status = 400, description = "Bad request", body = String, example = json!("Market with symbol <base_currency>/<quote_currency> already exists.")),
    ),
    tag = "Markets",
)]
#[post("/{base_currency}/{quote_currency}")]
async fn create(
    path: web::Path<(String, String)>,
    body: web::Json<PostRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Exception> {
    let (base_currency, quote_currency) = path.into_inner();
    let market = Mutation::create_market(
        &data.db,
        base_currency,
        quote_currency,
        body.price_increment.clone(),
        body.size_increment.clone()
    )
        .await
        .map_err(|e| Exception::Database(e))?;

    Ok(HttpResponse::Ok().json(market))
}

#[utoipa::path(
context_path = "/markets",
    params(
        ("id", description = "ID of the market to update")
    ),
    request_body = PutRequest,
    responses(
        (status = 200, description = "Returns none", body = None),
        (status = 500, description = "Internal server error", body = String, example = json!("An internal server error occurred. Please try again later.")),
        (status = 400, description = "Bad request", body = String, example = json!("Market with id <id> does not exist.")),
    ),
    tag = "Markets",
)]
#[put("/{id}")]
async fn update(
    path: web::Path<i32>,
    body: web::Json<PutRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Exception> {
    let id = path.into_inner();
    Mutation::update_market(
        &data.db,
        id,
        body.base_currency.clone(),
        body.quote_currency.clone(),
        body.price_increment.clone(),
        body.size_increment.clone(),
    )
        .await
        .map_err(|e| Exception::Database(e))?;

    Ok(HttpResponse::Ok().finish())
}

// ----------------------------------------------------------------------

#[derive(OpenApi)]
#[openapi(
    paths(get, get_by_symbol, create, update),
    components(schemas(Market)),
    tags((name = "Markets", description = "Market management endpoints.")),
)]
pub struct ApiDoc;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
    cfg.service(get_by_symbol);
    cfg.service(create);
    cfg.service(update);
}

// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use serde_json::json;
    use database::{Engine, Migrator, MigratorTrait};

    use super::*;

    #[actix_web::test]
    async fn main() {
        // Set up
        let db = Engine::connect().await.unwrap();
        let state = AppState { db: db.clone() }; // Build app state
        Migrator::refresh(&db).await.unwrap(); // Apply all pending migrations

        // Mock server
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state.clone()))
                .configure(router)
        ).await;

        // Get all
        let req = test::TestRequest::get()
            .uri("/")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Get one with error
        let req = test::TestRequest::get()
            .uri("/BTC/USD")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        // Create one
        let req = test::TestRequest::post()
            .uri("/BTC/USD")
            .set_json(json!({"price_increment": 0.01, "size_increment": 0.01}))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Create one with error
        let req = test::TestRequest::post()
            .uri("/BTC/USD")
            .set_json(json!({"price_increment": 0.01, "price_increment": 0.01}))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        // Get one
        let req = test::TestRequest::get()
            .uri("/BTC/USD")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Update one
        let req = test::TestRequest::put()
            .uri("/1")
            .set_json(json!({
                "base_currency": "BTC",
                "quote_currency": "USD",
                "price_increment": 0.01,
                "size_increment": 0.01
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Update one with error
        let req = test::TestRequest::put()
            .uri("/2")
            .set_json(json!({
                "base_currency": "BTC",
                "quote_currency": "USD",
                "price_increment": 0.01,
                "size_increment": 0.01
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        // Tear down
        Migrator::reset(&db).await.unwrap(); // Rollback migrations
    }
}