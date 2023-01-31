use actix_web::{post, web, HttpResponse};

use database::utoipa;
use database::utoipa::OpenApi;

use utoipa_swagger_ui::{SwaggerUi, Url};

use crate::AppState;

mod clients;
mod fills;
mod markets;
mod orders;
mod positions;
mod sub_accounts;

// ----------------------------------------------------------------------

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(shutdown),
    tags((name = "Index", description = "API Entrypoint.")),
)]
pub struct ApiDoc;

#[utoipa::path(
    context_path = "/",
    params(
        ("graceful", description = "Whether or not to forcefully or gracefully shutdown", example = true),
    ),
    responses(
        (status = 200, description = "Gracefully shuts down the server."),
        (status = 500, description = "Internal server error.", body = String, example = json!("An internal server error occurred. Please try again later.")),
    ),
    tag = "Index",
)]
#[post("/shutdown/{graceful}")]
async fn shutdown(path: web::Path<bool>, data: web::Data<AppState>) -> HttpResponse {
    let graceful = path.into_inner();
    let _ = &data.stop_handle.stop(graceful);
    HttpResponse::NoContent().finish()
}

// ----------------------------------------------------------------------

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/clients").configure(clients::router));
    cfg.service(web::scope("/markets").configure(markets::router));
    cfg.service(web::scope("/sub_accounts").configure(sub_accounts::router));
    cfg.service(web::scope("/orders").configure(orders::router));
    cfg.service(web::scope("/fills").configure(fills::router));
    cfg.service(web::scope("/positions").configure(positions::router));
    cfg.service(shutdown);
    cfg.service(SwaggerUi::new("/swagger/{_:.*}").urls(vec![
        (
            Url::with_primary("index", "/index-schema/openapi.json", true),
            ApiDoc::openapi(),
        ),
        (
            Url::new("clients", "/clients-schema/openapi.json"),
            clients::ApiDoc::openapi(),
        ),
        (
            Url::new("markets", "/markets-schema/openapi.json"),
            markets::ApiDoc::openapi(),
        ),
        (
            Url::new("orders", "/orders-schema/openapi.json"),
            orders::ApiDoc::openapi(),
        ),
        (
            Url::new("fills", "/fills-schema/openapi.json"),
            fills::ApiDoc::openapi(),
        ),
        (
            Url::new("positions", "/doc/positions-openapi.json"),
            positions::ApiDoc::openapi(),
        ),
        (
            Url::new("sub_accounts", "/doc/sub_accounts-openapi.json"),
            sub_accounts::ApiDoc::openapi(),
        ),
    ]));
}
