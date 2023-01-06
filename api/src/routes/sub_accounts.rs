use crate::models::{
    error::Exception,
    sub_account::{GetRequest, PostRequest, PutRequest, SubAccount},
};
use crate::AppState;

use actix_web::{get, post, put, web, HttpResponse};

use database::{Query, Mutation};

use utoipa::OpenApi;

// ----------------------------------------------------------------------

#[utoipa::path(
    context_path = "/sub_accounts",
    params(GetRequest),
    responses(
        (status = 200, description = "Returns all sub-accounts", body = [SubAccount]),
        (status = 500, description = "Internal server error", body = String, example = json!("An internal server error occurred. Please try again later.")),
    ),
    tag = "Sub-Accounts",
)]
#[get("/")]
async fn get(
    query: web::Query<GetRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Exception> {
    let sub_accounts = Query::find_sub_accounts(
        &data.db,
        query.status.clone(),
        query.page.clone(),
        query.page_size.clone(),
    )
        .await
        .map_err(|e| Exception::Database(e))?;

    Ok(HttpResponse::Ok().json(sub_accounts))
}

#[utoipa::path(
    context_path = "/sub_accounts",
    responses(
        (status = 200, description = "Returns all sub-accounts with the matching client id", body = [SubAccount]),
        (status = 500, description = "Internal server error", body = String, example = json!("An internal server error occurred. Please try again later.")),
        (status = 400, description = "Bad request", body = String, example = json!("Client with id <id> does not exist.")),
    ),
    params(
        ("client_id", description = "Client ID for which to search sub-accounts"),
    ),
    tag = "Sub-Accounts",
)]
#[get("/{client_id}")]
async fn get_by_client_id(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Exception> {
    let client_id = path.into_inner();
    let sub_accounts = Query::find_sub_accounts_by_client_id(&data.db, client_id)
        .await
        .map_err(|e| Exception::Database(e))?;

    Ok(HttpResponse::Ok().json(sub_accounts))
}

#[utoipa::path(
    context_path = "/sub_accounts",
    params(
        ("client_id", description = "The client id for which to create a new sub-account."),
    ),
    request_body = PostRequest,
    responses(
        (status = 200, description = "Returns the created sub-account record", body = SubAccount),
        (status = 500, description = "Internal server error", body = String, example = json!("An internal server error occurred. Please try again later.")),
        (status = 400, description = "Bad request", body = String, example = json!("Client with id <client_id> does not exist.")),
        (status = 400, description = "Bad request", body = String, example = json!("Sub-account with name <name> already exists.")),
    ),
    tag = "Sub-Accounts",
)]
#[post("/{client_id}")]
async fn create(
    path: web::Path<i32>,
    body: web::Json<PostRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Exception> {
    let client_id = path.into_inner();
    let sub_account = Mutation::create_sub_account(
        &data.db,
        client_id,
        body.name.clone(),
    )
        .await
        .map_err(|e| Exception::Database(e))?;

    Ok(HttpResponse::Ok().json(sub_account))
}

#[utoipa::path(
    context_path = "/sub_accounts",
    params(
        ("id", description = "ID of the sub-account to update")
    ),
    request_body = PutRequest,
    responses(
        (status = 200, description = "Returns none", body = None),
        (status = 500, description = "Internal server error", body = String, example = json!("An internal server error occurred. Please try again later.")),
        (status = 400, description = "Bad request", body = String, example = json!("Client with id <client_id> does not exist.")),
        (status = 400, description = "Bad request", body = String, example = json!("Sub-Account with id <id> does not exist.")),
    ),
    tag = "Sub-Accounts",
)]
#[put("/{client_id}")]
async fn update(
    path: web::Path<i32>,
    body: web::Json<PutRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Exception> {
    let client_id = path.into_inner();
    Mutation::update_sub_account(
        &data.db,
        client_id,
        body.id.clone(),
        body.name.clone(),
        body.status.clone(),
    )
        .await
        .map_err(|e| Exception::Database(e))?;

    Ok(HttpResponse::Ok().finish())
}

// ----------------------------------------------------------------------

#[derive(OpenApi)]
#[openapi(
    paths(get, get_by_client_id, create, update),
    components(schemas(SubAccount)),
    tags((name = "Sub-Accounts", description = "Sub-account management endpoints.")),
)]
pub struct ApiDoc;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
    cfg.service(get_by_client_id);
    cfg.service(create);
    cfg.service(update);
}

// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use serde_json::json;
    use database::{Engine, Migrator, MigratorTrait, SubAccountStatus};

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
        let _ = Mutation::create_client(&db, "b@gmail.com".to_owned()).await;

        // Create records
        let req = test::TestRequest::post()
            .uri("/1")
            .set_json(json!({"name": "Test1"}))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let req = test::TestRequest::post()
            .uri("/1")
            .set_json(json!({"name": "Test2"}))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let req = test::TestRequest::post()
            .uri("/2")
            .set_json(json!({"name": "Test1"}))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Get all
        let req = test::TestRequest::get()
            .uri("/")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Get some
        let req = test::TestRequest::get()
            .uri("/?status=Active&page=1&page_size=2")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Get all for client
        let req = test::TestRequest::get()
            .uri("/1")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Create one with error
        let req = test::TestRequest::post()
            .uri("/100")
            .set_json(json!({"name": "Test"}))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
        let req = test::TestRequest::post()
            .uri("/2")
            .set_json(json!({"name": "Test1"}))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        // Update one
        let req = test::TestRequest::put()
            .uri("/1")
            .set_json(json!({
                "id": 1,
                "name": "Test100",
                "status": SubAccountStatus::Inactive,
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Update one with error
        let req = test::TestRequest::put()
            .uri("/100")
            .set_json(json!({
                "id": 1,
                "name": "Test",
                "status": SubAccountStatus::Inactive,
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
        let req = test::TestRequest::put()
            .uri("/1")
            .set_json(json!({
                "id": 100,
                "name": "Test2",
                "status": SubAccountStatus::Inactive,
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        // Tear down
        Migrator::reset(&db).await.unwrap(); // Rollback migrations
    }
}
