use crate::models::Exception;
use crate::AppState;

use actix_web::{get, web, HttpResponse, Responder};
use common::rabbitmq::Stream;
use database::fills::{ClientGetRequest, Response};
use database::{utoipa, OrderSide, OrderType, Query};

// ----------------------------------------------------------------------

#[utoipa::path(
    context_path = "/fills",
    params(
        ("client_id", description = "Client ID for which to search fills.", example = 1),
        ClientGetRequest
    ),
    responses(
        (status = 200, description = "Returns all positions.", body = [Response]),
        (status = 500, description = "Internal server error.", body = String, example = json!("An internal server error occurred. Please try again later.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Sub-account with id <sub_account_id> does not exist.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Sub-account with name <sub_account_name> does not exist.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Client with id <client_id> does not exist.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Market with id <market_id> does not exist.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Market with base currency <base_currency> does not exist.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Market with quote currency <quote_currency> does not exist.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Market with base currency <base_currency> and quote currency <quote_currency> does not exist.")),
    ),
    tag = "Fills",
)]
#[get("/{client_id}")]
async fn get_client_related(
    path: web::Path<i32>,
    query: web::Query<ClientGetRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Exception> {
    let client_id = path.into_inner();
    let fills = Query::find_client_related_fills(
        &data.db,
        client_id,
        query.sub_account_id,
        query.sub_account_name.clone(),
        query.market_id,
        query.base_currency.clone(),
        query.quote_currency.clone(),
        query.order_id,
        query.side.clone(),
        query.r#type.clone(),
        query.start_time,
        query.end_time,
        query.page,
        query.page_size,
    )
    .await
    .map_err(Exception::Database)?;

    Ok(HttpResponse::Ok().json(fills))
}

#[utoipa::path(
    context_path = "/fills",
    responses(
        (status = 200, description = "Returns a SSE streaming connection."),
        (status = 500, description = "Internal server error.", body = String, example = json!("An internal server error occurred. Please try again later.")),
    ),
    tag = "Fills",
)]
#[get("/stream")]
async fn stream(data: web::Data<AppState>) -> impl Responder {
    data.broadcaster.new_client(Stream::Fills).await
}

// ----------------------------------------------------------------------

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(get_client_related, stream),
    components(schemas(Response, OrderSide, OrderType)),
    tags((name = "Fills", description = "Fill management endpoints.")),
)]
pub struct ApiDoc;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(get_client_related);
    cfg.service(stream);
}

// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::jobs::Broadcaster;
    use crate::StopHandle;
    use actix_web::{test, App};
    use database::{Engine, Migrator, MigratorTrait, Mutation};

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

        // Get all for client with error
        let req = test::TestRequest::get().uri("/100").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        // Get all for client
        let req = test::TestRequest::get().uri("/1").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Tear down
        Migrator::reset(&db).await.unwrap(); // Rollback migrations
    }
}
