use chrono::{DateTime, Utc};
use database::{OrderSide, Fill as FillModel, OrderType};

use utoipa::{IntoParams, ToSchema};

use serde::Deserialize;
// TODO: Rename Position to model
// ----------------------------------------------------------------------

pub struct Fill(FillModel);

impl ToSchema for Fill {
    fn schema() -> utoipa::openapi::schema::Schema {
        utoipa::openapi::ObjectBuilder::new()
            .property(
                "price",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::Number)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::Float,
                    ))),
            )
            .required("price")
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
                "quote_size",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::Number)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::Float,
                    ))),
            )
            .required("quote_size")
            .property(
                "side",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::String)
                    .enum_values(Some(vec!["buy", "sell"])),
            )
            .required("side")
            .property(
                "type",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::String)
                    .enum_values(Some(vec!["Limit", "Market"])),
            )
            .required("type")
            .property(
                "created_at",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::String)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::DateTime,
                    ))),
            )
            .required("created_at")
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
                "order_id",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::Integer)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::Int32,
                    ))),
            )
            .required("order_id")
            .example(Some(serde_json::json!({
                "price": 100.0,
                "size": 100.0,
                "quote_size": 100.0,
                "side": "Buy",
                "type": "Limit",
                "created_at": "2022-01-01T00:00:00",
                "base_currency": "BTC",
                "quote_currency": "USD",
                "price_increment": 0.01,
                "size_increment": 0.01,
                "sub_account": "Test",
                "order_id": 1,
            })))
            .into()
    }
}

#[derive(Deserialize, IntoParams)]
pub struct ClientGetRequest {
    pub sub_account_id: Option<i32>,
    pub sub_account_name: Option<String>,
    pub market_id: Option<i32>,
    pub base_currency: Option<String>,
    pub quote_currency: Option<String>,
    pub order_id: Option<i32>,
    pub side: Option<OrderSide>,
    pub r#type: Option<OrderType>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}
