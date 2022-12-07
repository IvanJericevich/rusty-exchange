use database::Position as BasePosition;

use utoipa::{IntoParams, ToSchema};

use serde::Deserialize;

// ----------------------------------------------------------------------

pub struct Position(BasePosition);

impl ToSchema for Position {
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
                "avg_entry_price",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::Number)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::Float,
                    ))),
            )
            .required("avg_entry_price")
            .property(
                "size",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::Number)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::Float,
                    ))),
            )
            .required("size")
            .property(
                "side",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::String)
                    .enum_values(Some(vec!["buy", "sell"]))
            )
            .required("side")
            .property(
                "sub_account_id",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::Integer)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::Int32,
                    ))),
            )
            .required("sub_account_id")
            .property(
                "market_id",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::Integer)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::Int32,
                    ))),
            )
            .required("market_id")
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
                "sub_account",
                utoipa::openapi::Object::with_type(utoipa::openapi::SchemaType::String),
            )
            .required("sub_account")
            .property(
                "client_id",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::Integer)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::Int32,
                    ))),
            )
            .required("client_id")
            .example(Some(serde_json::json!({
                "id": 1,
                "avg_entry_price": 1.056,
                "size": 100,
                "side": "Buy",
                "sub_account_id": 1,
                "market_id": 1,
                "base_currency": "BTC",
                "quote_currency": "USD",
                "price_increment": 0.01,
                "size_increment": 0.01,
                "sub_account": "Test",
                "client_id": 1,
            })))
            .into()
    }
}

#[derive(Deserialize, IntoParams)]
pub struct Request {
    pub sub_account: Option<String>,
    pub base_currency: Option<String>,
    pub quote_currency: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
