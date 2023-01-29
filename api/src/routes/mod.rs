use actix_web::web;

use database::utoipa::OpenApi;

use utoipa_swagger_ui::{SwaggerUi, Url};

mod clients;
mod fills;
mod markets;
mod orders;
mod positions;
mod sub_accounts;

// ----------------------------------------------------------------------

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/clients").configure(clients::router));
    cfg.service(web::scope("/markets").configure(markets::router));
    cfg.service(web::scope("/sub_accounts").configure(sub_accounts::router));
    cfg.service(web::scope("/orders").configure(orders::router));
    cfg.service(web::scope("/fills").configure(fills::router));
    cfg.service(web::scope("/positions").configure(positions::router));
    cfg.service(SwaggerUi::new("/swagger/{_:.*}").urls(vec![
        (
            Url::with_primary("clients", "/clients-schema/openapi.json", true),
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
