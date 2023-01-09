use crate::models::error::Exception;
use crate::AppState;

use actix_web::{get, web, HttpResponse};
use database::fills::{Response, ClientGetRequest};

use database::utoipa;
use database::Query;

// ----------------------------------------------------------------------

#[utoipa::path(
    context_path = "/fills",
    params(
        ("client_id", description = "Client ID for which to search fills."),
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
        query.sub_account_id.clone(),
        query.sub_account_name.clone(),
        query.market_id.clone(),
        query.base_currency.clone(),
        query.quote_currency.clone(),
        query.order_id.clone(),
        query.side.clone(),
        query.r#type.clone(),
        query.start_time.clone(),
        query.end_time.clone(),
        query.page.clone(),
        query.page_size.clone()
    )
        .await
        .map_err(|e| Exception::Database(e))?;

    Ok(HttpResponse::Ok().json(fills))
}

// ----------------------------------------------------------------------

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(get_client_related),
    components(schemas(Response)),
    tags((name = "Fills", description = "Fill management endpoints.")),
)]
pub struct ApiDoc;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(get_client_related);
}

// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use database::{Engine, Migrator, MigratorTrait, Mutation};

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
        let _ = Mutation::create_client(&db, "a@gmail.com".to_owned()).await;

        // Get all for client with error
        let req = test::TestRequest::get()
            .uri("/100")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        // Get all for client
        let req = test::TestRequest::get()
            .uri("/1")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Tear down
        Migrator::reset(&db).await.unwrap(); // Rollback migrations
    }
}