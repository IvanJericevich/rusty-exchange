use database::MarketModel;

use utoipa::{IntoParams, ToSchema};

use serde::Deserialize;

// ----------------------------------------------------------------------

pub struct Market(MarketModel);

impl ToSchema for Market {
    fn schema() -> utoipa::openapi::schema::Schema {
        utoipa::openapi::ObjectBuilder::new()
            .property(
                "id",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::Integer)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::Int32,
                    ))),
            )
            .required("id")
            .property(
                "base_currency",
                utoipa::openapi::Object::with_type(utoipa::openapi::SchemaType::String),
            )
            .required("base_currency")
            .property(
                "quote_currency",
                utoipa::openapi::Object::with_type(utoipa::openapi::SchemaType::String),
            )
            .required("quote_currency")
            .property(
                "price_increment",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::Number)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::Float,
                    ))),
            )
            .required("price_increment")
            .property(
                "size_increment",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::Number)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::Float,
                    ))),
            )
            .required("size_increment")
            .property(
                "created_at",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::String)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::DateTime,
                    ))),
            )
            .required("created_at")
            .example(Some(serde_json::json!({
                "id": 1,
                "base_currency": "BTC",
                "quote_currency": "USD",
                "price_increment": 0.01,
                "size_increment": 0.01,
                "created_at": "2022-01-01T00:00:00"
            })))
            .into()
    }
}

#[derive(Deserialize, IntoParams)]
pub struct Request {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
