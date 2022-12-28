use crate::models::{
    error::Exception,
    position::{Position, ClientGetRequest},
};
use crate::AppState;

use actix_web::{get, web, HttpResponse};

use utoipa::OpenApi;
use database::Query;

// ----------------------------------------------------------------------

#[utoipa::path(
    context_path = "/orders",
    params(
        ("client_id", description = "Client ID for which to search positions."),
        ClientGetRequest
    ),
    responses(
        (status = 200, description = "Returns all positions.", body = [Position]),
        (status = 500, description = "Internal server error.", body = String, example = json!("An internal server error occurred. Please try again later.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Sub-account with id <sub_account_id> does not exist.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Client with id <client_id> does not exist.")),
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
    let positions = Query::find_client_related_positions(
        &data.db,
        client_id,
        query.sub_account_id.clone(),
        query.market_id.clone(),
        query.side.clone(),
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
    paths(get_client_related),
    components(schemas(Position)),
    tags((name = "Positions", description = "Position management endpoints.")),
)]
pub struct ApiDoc;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(get_client_related);
}

// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use database::{Engine, Migrator, MigratorTrait, Mutation, SubAccountStatus};

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

        // Get all for client with error
        let req = test::TestRequest::get()
            .uri("/1")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        // Get all for client
        let _ = Mutation::create_client(&db, "ivanjericevich96@gmail.com".to_owned()).await;
        let req = test::TestRequest::get()
            .uri("/1")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Tear down
        Migrator::reset(&db).await.unwrap(); // Rollback migrations
    }
}