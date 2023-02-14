use actix_web::{get, HttpResponse, post, web};
use rabbitmq_stream_client::types::Message;

use database::{Mutation, OrderSide, OrderStatus, OrderType, Query, utoipa};
use database::orders::{ClientGetOpenRequest, ClientGetRequest, Order, PostRequest, Response};

use crate::AppState;
use crate::models::Exception;

// ----------------------------------------------------------------------

#[utoipa::path(
context_path = "/orders",
params(
("client_id", description = "Client ID for which to search orders.", example = 1),
ClientGetOpenRequest
),
responses(
        (status = 200, description = "Returns all orders.", body = Response),
        (status = 500, description = "Internal server error.", body = String, example = json!("An internal server error occurred. Please try again later.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Sub-account with id <sub_account_id> does not exist.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Sub-account with name <sub_account_name> does not exist.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Client with id <client_id> does not exist.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Market with id <market_id> does not exist.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Market with base currency <base_currency> does not exist.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Market with quote currency <quote_currency> does not exist.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Market with base currency <base_currency> and quote currency <quote_currency> does not exist.")),
    ),
    tag = "Orders",
)]
#[get("/open/{client_id}")]
async fn get_client_related_open(
    path: web::Path<i32>,
    query: web::Query<ClientGetOpenRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Exception> {
    let client_id = path.into_inner();
    let order = Query::find_client_related_open_order(
        &data.db,
        client_id,
        query.id,
        query.client_order_id.clone(),
        query.sub_account_id,
        query.sub_account_name.clone(),
        query.market_id,
        query.base_currency.clone(),
        query.quote_currency.clone(),
        query.side.clone(),
    )
    .await
    .map_err(Exception::Database)?;

    Ok(HttpResponse::Ok().json(order))
}

#[utoipa::path(
    context_path = "/orders",
    params(
        ("client_id", description = "Client ID for which to search orders.", example = 1),
        ClientGetRequest
    ),
    responses(
        (status = 200, description = "Returns all orders.", body = [Response]),
        (status = 500, description = "Internal server error.", body = String, example = json!("An internal server error occurred. Please try again later.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Sub-account with id <sub_account_id> does not exist.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Sub-account with name <sub_account_name> does not exist.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Client with id <client_id> does not exist.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Market with id <market_id> does not exist.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Market with base currency <base_currency> does not exist.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Market with quote currency <quote_currency> does not exist.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Market with base currency <base_currency> and quote currency <quote_currency> does not exist.")),
    ),
    tag = "Orders",
)]
#[get("/{client_id}")]
async fn get_client_related(
    path: web::Path<i32>,
    query: web::Query<ClientGetRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Exception> {
    let client_id = path.into_inner();
    let orders = Query::find_client_related_orders(
        &data.db,
        client_id,
        query.sub_account_id,
        query.sub_account_name.clone(),
        query.market_id,
        query.base_currency.clone(),
        query.quote_currency.clone(),
        query.client_order_id.clone(),
        query.side.clone(),
        query.r#type.clone(),
        query.status.clone(),
        query.start_time,
        query.end_time,
        query.page,
        query.page_size,
    )
    .await
    .map_err(Exception::Database)?;

    Ok(HttpResponse::Ok().json(orders))
}

// #[utoipa::path(
//     context_path = "/orders",
//     params(
//         ("market_id", description = "Market ID for which to search orders."),
//         MarketGetRequest
//     ),
//     responses(
//         (status = 200, description = "Returns all orders.", body = [Order]),
//         (status = 500, description = "Internal server error.", body = String, example = json!("An internal server error occurred. Please try again later.")),
//         (status = 400, description = "Bad request.", body = String, example = json!("Market with id <market_id> does not exist.")),
//     ),
//     tag = "Orders",
// )]
// #[get("/{market_id}")]
// async fn get_market_related(
//     path: web::Path<i32>,
//     query: web::Query<MarketGetRequest>,
//     data: web::Data<AppState>,
// ) -> Result<HttpResponse, Exception> {
//     let market_id = path.into_inner();
//     let orders = Query::find_market_related_orders(
//         &data.db,
//         market_id,
//         query.side.clone(),
//         query.r#type.clone(),
//         query.status.clone(),
//         query.start_time.clone(),
//         query.end_time.clone(),
//         query.page.clone(),
//         query.page_size.clone()
//     )
//         .await
//         .map_err(|e| Exception::Database(e))?;
//
//     Ok(HttpResponse::Ok().json(orders))
// }

#[utoipa::path(
    context_path = "/orders",
    params(
        ("client_id", description = "The client id for which to create a new sub-account.", example = 1),
    ),
    request_body = PostRequest,
    responses(
        (status = 200, description = "Returns the created order record", body = Order),
        (status = 500, description = "Internal server error", body = String, example = json!("An internal server error occurred. Please try again later.")),
        (status = 400, description = "Bad request", body = String, example = json!("Missing query arguments.")),
        (status = 400, description = "Bad request", body = String, example = json!("Invalid order parameters.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Sub-account with id <sub_account_id> does not exist.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Client with id <client_id> does not exist.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Market does not exist.")),
    ),
    tag = "Orders",
)]
#[post("/{client_id}")]
async fn create(
    path: web::Path<i32>,
    body: web::Json<PostRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Exception> {
    let client_id = path.into_inner();
    let order = Mutation::create_order(
        &data.db,
        client_id,
        body.sub_account_id,
        body.size,
        body.side.clone(),
        body.r#type.clone(),
        body.price,
        body.client_order_id.clone(),
        body.market_id,
        body.base_currency.clone(),
        body.quote_currency.clone(),
    )
    .await
    .map_err(Exception::Database)?;

    if let Some(producer) = &data.producer {
        let _ = producer
            .send_with_confirm(
                Message::builder()
                    .body(serde_json::to_string(&order).unwrap())
                    .build(),
            )
            .await
            .map_err(Exception::RabbitMQ)?;
    }

    Ok(HttpResponse::Ok().json(order))
}

// ----------------------------------------------------------------------

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(get_client_related_open, get_client_related, create),
    components(schemas(Response, PostRequest, Order, OrderSide, OrderType, OrderStatus)),
    tags((name = "Orders", description = "Order management endpoints.")),
)]
pub struct ApiDoc;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(get_client_related_open);
    cfg.service(get_client_related);
    cfg.service(create);
}

// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use actix_web::{App, test};
    use serde_json::json;

    use database::{Engine, Migrator, MigratorTrait, Mutation, OrderSide, OrderType};

    use crate::jobs::Broadcaster;
    use crate::StopHandle;

    use super::*;

    #[actix_web::test]
    async fn main() {
        // Set up
        let db = Engine::connect().await.unwrap();
        let state = web::Data::new(AppState {
            db: db.clone(),
            producer: None,
            stop_handle: StopHandle::default(),
            broadcaster: Broadcaster::create(),
        }); // Build app state
        Migrator::refresh(&db).await.unwrap(); // Apply all pending migrations

        // Mock server
        let app = test::init_service(App::new().app_data(state.clone()).configure(router)).await;
        let _ = Mutation::create_client(&db, "a@gmail.com".to_owned()).await;
        let _ = Mutation::create_sub_account(&db, 1, "Test".to_owned()).await;
        let _ = Mutation::create_market(&db, "BTC".to_owned(), "USD".to_owned(), 0.01, 0.01).await;
        let _ = Mutation::create_market(&db, "ETH".to_owned(), "USD".to_owned(), 0.01, 0.01).await;
        // Create records
        let req = test::TestRequest::post()
            .uri("/1")
            .set_json(json!({
                "sub_account_id": 1,
                "size": 100.0,
                "side": OrderSide::Buy,
                "type": OrderType::Limit,
                "price": 100.0,
                "client_order_id": "abc123",
                "market_id": 1,
                "base_currency": "BTC",
                "quote_currency": "USD",
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let req = test::TestRequest::post()
            .uri("/1")
            .set_json(json!({
                "sub_account_id": 1,
                "size": 100.0,
                "side": OrderSide::Buy,
                "type": OrderType::Limit,
                "price": 100.0,
                "client_order_id": "abc123",
                "market_id": 1,
                "base_currency": "ETH",
                "quote_currency": "USD",
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Get all for client with error
        let req = test::TestRequest::get().uri("/2").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        // Get all for client
        let req = test::TestRequest::get().uri("/1").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Get some for client
        let req = test::TestRequest::get()
            .uri("/1?sub_account_id=1&market_id=1&side=Buy&type=Limit&status=Closed")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let req = test::TestRequest::get()
            .uri("/1?sub_account_name=Test&base_currency=BTC&quote_currency=USD&side=Buy&type=Limit&status=Open")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Get all open for client
        let req = test::TestRequest::get().uri("/open/1").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Tear down
        Migrator::reset(&db).await.unwrap(); // Rollback migrations
    }
}
