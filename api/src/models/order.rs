use chrono::{DateTime, Utc};
use database::{
    Order as OrderModel, OrderSide, OrderStatus, OrderType,
};

use utoipa::{IntoParams, ToSchema};

use serde::Deserialize;

// ----------------------------------------------------------------------

pub struct Order(OrderModel);

impl ToSchema for Order {
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
                "filled_size",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::Number)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::Float,
                    ))),
            )
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
                    .enum_values(Some(vec!["limit", "market"])),
            )
            .required("type")
            .property(
                "status",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::String)
                    .enum_values(Some(vec!["closed", "open"])),
            )
            .required("status")
            .property(
                "open_at",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::String)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::DateTime,
                    ))),
            )
            .required("open_at")
            .property(
                "closed_at",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::SchemaType::String)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::DateTime,
                    ))),
            )
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
                "price": 100,
                "size": 100,
                "filled_size": 30,
                "side": OrderSide::Buy,
                "type": OrderType::Limit,
                "status": OrderStatus::Open,
                "open_at": "2022-01-01T00:00:00",
                "closed_at": null,
                "sub_account_id": 1,
                "market_id": 1,
                "base_currency": "BTC",
                "quote_currency": "USD",
                "price_increment": 0.01,
                "size_increment": 0.01,
                "sub_account": "Test",
                "client_id": 1
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
    pub client_order_id: Option<String>,
    pub side: Option<OrderSide>,
    pub r#type: Option<OrderType>,
    pub status: Option<OrderStatus>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Deserialize, IntoParams)]
pub struct ClientGetOpenRequest {
    pub id: Option<i32>,
    pub client_order_id: Option<String>,
    pub sub_account_id: Option<i32>,
    pub sub_account_name: Option<String>,
    pub market_id: Option<i32>,
    pub base_currency: Option<String>,
    pub quote_currency: Option<String>,
    pub side: Option<OrderSide>,
}

#[derive(Deserialize, IntoParams)]
pub struct MarketGetRequest {
    pub side: Option<OrderSide>,
    pub r#type: Option<OrderType>,
    pub status: Option<OrderStatus>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Deserialize, IntoParams)]
pub struct PostRequest {
    pub sub_account_id: i32,
    pub size: f32,
    pub side: OrderSide,
    pub r#type: OrderType,
    pub price: Option<f32>,
    pub client_order_id: Option<String>,
    pub market_id: Option<i32>,
    pub base_currency: Option<String>,
    pub quote_currency: Option<String>,
}
