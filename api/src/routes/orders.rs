use crate::models::{order::{Order, Request}, error::Exception};
use crate::AppState;

use actix_web::{get, web, HttpResponse};

use database::Query;

use utoipa::OpenApi;

// ----------------------------------------------------------------------

#[utoipa::path(
    context_path = "/sub_accounts",
    params(Request),
    responses(
        (status = 200, description = "Returns all orders", body = [Order]),
        (status = 500, description = "Internal server error", body = String, example = json!(String::from("An internal server error occurred. Please try again later."))),
    ),
)]
#[get("")]
async fn index(query: web::Query<Request>, data: web::Data<AppState>) -> Result<HttpResponse, Exception> {

    let orders = Query::find_orders(
        &data.db,
        query.side.clone(),
        query.r#type.clone(),
        query.sub_account.clone(),
        query.client_id.clone(),
        query.status.clone(),
        query.base_currency.clone(),
        query.quote_currency.clone(),
        None,
        None,
        query.page.clone(),
        query.page_size.clone()
    )
        .await
        .map_err(|e| Exception::Database(e))?;

    Ok(HttpResponse::Ok().json(orders))
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
