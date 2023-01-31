use crate::models::Exception;
use crate::AppState;

use actix_web::{get, post, put, web, HttpResponse};

use database::clients::{GetRequest, Model, PutRequest};
use database::utoipa;
use database::{Mutation, Query};

// ----------------------------------------------------------------------

#[utoipa::path(
    context_path = "/clients",
    params(GetRequest),
    responses(
        (status = 200, description = "Returns all clients.", body = [Model]),
        (status = 500, description = "Internal server error.", body = String, example = json!("An internal server error occurred. Please try again later.")),
    ),
    tag = "Clients",
)]
#[get("/")]
async fn get(
    query: web::Query<GetRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Exception> {
    let clients = Query::find_clients(&data.db, query.page, query.page_size)
        .await
        .map_err(Exception::Database)?;

    Ok(HttpResponse::Ok().json(clients))
}

#[utoipa::path(
    context_path = "/clients",
    responses(
        (status = 200, description = "Returns a client with the matching email address.", body = Model),
        (status = 500, description = "Internal server error.", body = String, example = json!("An internal server error occurred. Please try again later.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Client with email <email> does not exist.")),
    ),
    params(
        ("email", description = "Email of the client to search for.", example = "example@gmail.com")
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
        .map_err(Exception::Database)?;

    Ok(HttpResponse::Ok().json(client))
}

// ----------------------------------------------------------------------

#[utoipa::path(
    context_path = "/clients",
    params(("email", description = "Email of the new client.", example = "example@gmail.com")),
    responses(
        (status = 201, description = "Returns the created client record.", body = Model),
        (status = 500, description = "Internal server error.", body = String, example = json!("An internal server error occurred. Please try again later.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Client with email <email> already exists.")),
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
        .map_err(Exception::Database)?;

    Ok(HttpResponse::Ok().json(client))
}

#[utoipa::path(
    context_path = "/clients",
    params(
        ("id", description = "ID of the client to update.", example = 1)
    ),
    request_body = PutRequest,
    responses(
        (status = 200, description = "Returns null."),
        (status = 500, description = "Internal server error.", body = String, example = json!("An internal server error occurred. Please try again later.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Client with id <id> does not exist.")),
        (status = 400, description = "Bad request.", body = String, example = json!("Client with email <new_email> already exists.")),
    ),
    tag = "Clients",
)]
#[put("/{id}")]
async fn update(
    path: web::Path<i32>,
    body: web::Query<PutRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Exception> {
    let id = path.into_inner();
    Mutation::update_client(&data.db, id, body.new_email.clone())
        .await
        .map_err(Exception::Database)?;

    Ok(HttpResponse::Ok().finish())
}

// ----------------------------------------------------------------------

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(get, get_by_email, create, update),
    components(schemas(Model, PutRequest)),
    tags((name = "Clients", description = "Client management endpoints.")),
)]
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
    use crate::jobs::Broadcaster;
    use crate::StopHandle;
    use actix_web::{test, App};
    use database::{Engine, Migrator, MigratorTrait};

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

        // Create records
        let req = test::TestRequest::post().uri("/a@gmail.com").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let req = test::TestRequest::post().uri("/b@gmail.com").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let req = test::TestRequest::post().uri("/c@gmail.com").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        // Create one with error
        let req = test::TestRequest::post().uri("/a@gmail.com").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        // Get all
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Get some
        let req = test::TestRequest::get()
            .uri("/?page=1&page_size=2")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Get one
        let req = test::TestRequest::get().uri("/a@gmail.com").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Get one with error
        let req = test::TestRequest::get()
            .uri("/unknown@gmail.com")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        // Update one
        let req = test::TestRequest::put()
            .uri("/1?new_email=joe@gmail.com")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Update one with error
        let req = test::TestRequest::put()
            .uri("/100?new_email=joe@gmail.com")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
        let req = test::TestRequest::put()
            .uri("/1?new_email=joe@gmail.com")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        // Tear down
        Migrator::reset(&db).await.unwrap(); // Rollback migrations
    }
}
