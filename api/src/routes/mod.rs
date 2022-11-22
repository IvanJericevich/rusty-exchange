use actix_web::web;

use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

mod clients;
mod markets;
mod orders;
mod positions;
mod sub_accounts;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/clients").configure(clients::router));
    cfg.service(web::scope("/markets").configure(markets::router));
    cfg.service(web::scope("/sub_accounts").configure(sub_accounts::router));
    cfg.service(web::scope("/orders").configure(orders::router));
    cfg.service(web::scope("/positions").configure(positions::router));
    cfg.service(SwaggerUi::new("/swagger/{_:.*}").urls(vec![
        (
            Url::with_primary("clients", "/doc/clients-openapi.json", true),
            clients::ApiDoc::openapi(),
        ),
        (
            Url::new("markets", "/doc/markets-openapi.json"),
            markets::ApiDoc::openapi(),
        ),
        (
            Url::new("orders", "/doc/orders-openapi.json"),
            orders::ApiDoc::openapi(),
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
