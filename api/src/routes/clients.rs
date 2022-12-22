use crate::models::{
    client::{Client, GetRequest, PutRequest},
    error::Exception,
};
use crate::AppState;

use actix_web::{get, post, put, web, HttpResponse};

use database::{Mutation, Query};

use utoipa::OpenApi;

// ----------------------------------------------------------------------

#[utoipa::path(
    context_path = "/clients",
    params(GetRequest),
    responses(
        (status = 200, description = "Returns all clients", body = [Client]),
        (status = 500, description = "Internal server error", body = String, example = json!("An internal server error occurred. Please try again later.")),
    ),
    tag = "Clients",
)]
#[get("/")]
async fn get(
    query: web::Query<GetRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Exception> {
    let clients = Query::find_clients(&data.db, query.page.clone(), query.page_size.clone())
        .await
        .map_err(|e| Exception::Database(e))?;

    Ok(HttpResponse::Ok().json(clients))
}

#[utoipa::path(
    context_path = "/clients",
    responses(
        (status = 200, description = "Returns a client with the matching email address", body = Client),
        (status = 500, description = "Internal server error", body = String, example = json!("An internal server error occurred. Please try again later.")),
        (status = 400, description = "Bad request", body = String, example = json!("Client with email <email> does not exist.")),
    ),
    params(
        ("email", description = "Email of the client to search for")
    ),
    tag = "Clients",
)]
#[get("/{email}")]
async fn get_by_email(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Exception> {
    let email = path.into_inner();
    let client = Query::find_client_by_email(&data.db, email.clone())
        .await
        .map_err(|e| Exception::Database(e))?;

    Ok(HttpResponse::Ok().json(client))
}

// ----------------------------------------------------------------------

#[utoipa::path(
    context_path = "/clients",
    params(GetRequest),
    responses(
        (status = 201, description = "Returns the created client record", body = Client),
        (status = 500, description = "Internal server error", body = String, example = json!("An internal server error occurred. Please try again later.")),
        (status = 400, description = "Bad request", body = String, example = json!("Client with email <email> already exists.")),
    ),
    tag = "Clients",
)]
#[post("/{email}")]
async fn create(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Exception> {
    let email = path.into_inner();
    let client = Mutation::create_client(&data.db, email)
        .await
        .map_err(|e| Exception::Database(e))?;

    Ok(HttpResponse::Ok().json(client))
}

#[utoipa::path(
    context_path = "/clients",
    params(
        PutRequest,
        ("id", description = "ID of the client to update")
    ),
    responses(
        (status = 200, description = "Returns none", body = None),
        (status = 500, description = "Internal server error", body = String, example = json!("An internal server error occurred. Please try again later.")),
        (status = 400, description = "Bad request", body = String, example = json!("Client with id <id> does not exist.")),
        (status = 400, description = "Bad request", body = String, example = json!("Client with email <new_email> already exists.")),
    ),
    tag = "Clients",
)]
#[put("/{id}")]
async fn update(
    path: web::Path<i32>,
    query: web::Query<PutRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Exception> {
    let id = path.into_inner();
    Mutation::update_client(&data.db, id, query.new_email.clone())
        .await
        .map_err(|e| Exception::Database(e))?;

    Ok(HttpResponse::Ok().finish())
}

// ----------------------------------------------------------------------

#[derive(OpenApi)]
#[openapi(paths(get_by_email, get, create, update), components(schemas(Client)))]
pub struct ApiDoc;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
    cfg.service(get_by_email);
    cfg.service(create);
    cfg.service(update);
}

// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use database::{Engine, Migrator, MigratorTrait};

    use super::*;

    #[actix_web::test]
    async fn main() {
        // Set up
        let db = Engine::connect().await.unwrap();
        let state = AppState { db: db.clone() }; // Build app state
        Migrator::up(&db, None).await.unwrap(); // Apply all pending migrations

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
            .uri("/ivanjericevich96@gmail.com")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        // Create one
        let req = test::TestRequest::post()
            .uri("/ivanjericevich96@gmail.com")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Create one with error
        let req = test::TestRequest::post()
            .uri("/ivanjericevich96@gmail.com")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        // Get one
        let req = test::TestRequest::get()
            .uri("/ivanjericevich96@gmail.com")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Update one
        let req = test::TestRequest::put()
            .uri("/1?new_email=joe@gmail.com")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Update one with error
        let req = test::TestRequest::put()
            .uri("/2?new_email=joe@gmail.com")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        // Update one with error
        let req = test::TestRequest::put()
            .uri("/1?new_email=joe@gmail.com")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        // Tear down
        Migrator::reset(&db).await.unwrap(); // Rollback migrations
    }
}
