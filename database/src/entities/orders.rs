//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.2

use super::sea_orm_active_enums::OrderSide;
use super::sea_orm_active_enums::OrderStatus;
use super::sea_orm_active_enums::OrderType;
use sea_orm::{FromQueryResult};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, IntoParams};

// ----------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[schema(example = 1)]
    pub id: i32,
    #[schema(example = "Example")]
    pub client_order_id: Option<String>,
    #[schema(example = 50.0)]
    pub price: Option<f32>,
    #[schema(example = 100.0)]
    pub size: f32,
    #[schema(example = 100.0)]
    pub filled_size: f32,
    #[schema(example = OrderSide::Buy)]
    pub side: OrderSide,
    #[schema(example = OrderType::Market)]
    pub r#type: OrderType,
    #[schema(example = OrderStatus::Closed)]
    pub status: OrderStatus,
    #[schema(example = "1970-01-01T00:00:00")]
    pub open_at: DateTime,
    #[schema(example = "1970-01-01T00:00:00")]
    pub closed_at: Option<DateTime>,
    #[schema(example = 1)]
    pub sub_account_id: i32,
    #[schema(example = 1)]
    pub market_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::markets::Entity",
        from = "Column::MarketId",
        to = "super::markets::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Markets,
    #[sea_orm(
        belongs_to = "super::sub_accounts::Entity",
        from = "Column::SubAccountId",
        to = "super::sub_accounts::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SubAccounts,
    #[sea_orm(has_many = "super::fills::Entity")]
    Fills,
}

impl Related<super::markets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Markets.def()
    }
}

impl Related<super::sub_accounts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SubAccounts.def()
    }
}

impl Related<super::fills::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Fills.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// ----------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, FromQueryResult, Serialize, ToSchema)]
pub struct Response {
    #[schema(example = "Test")]
    pub client_order_id: Option<String>,
    #[schema(example = 50.0)]
    pub price: Option<f32>,
    #[schema(example = 100.0)]
    pub size: f32,
    #[schema(example = 100.0)]
    pub filled_size: f32,
    #[schema(example = OrderSide::Buy)]
    pub side: OrderSide,
    #[schema(example = OrderType::Market)]
    pub r#type: OrderType,
    #[schema(example = OrderStatus::Closed)]
    pub status: OrderStatus,
    #[schema(example = "1970-01-01T00:00:00")]
    pub open_at: DateTime,
    #[schema(example = "1970-01-01T00:00:00")]
    pub closed_at: Option<DateTime>,
    #[schema(example = "BTC")]
    pub base_currency: String,
    #[schema(example = "USD")]
    pub quote_currency: String,
    #[schema(example = 0.01)]
    pub price_increment: f32,
    #[schema(example = 0.01)]
    pub size_increment: f32,
    #[schema(example = "Test")]
    pub sub_account: String,
}

#[derive(Deserialize, IntoParams)]
pub struct ClientGetRequest {
    #[param(example = 1)]
    pub sub_account_id: Option<i32>,
    #[param(example = "Test")]
    pub sub_account_name: Option<String>,
    #[param(example = 1)]
    pub market_id: Option<i32>,
    #[param(example = "BTC")]
    pub base_currency: Option<String>,
    #[param(example = "USD")]
    pub quote_currency: Option<String>,
    #[param(example = "Test")]
    pub client_order_id: Option<String>,
    #[param(example = "Buy")]
    pub side: Option<OrderSide>,
    #[param(example = "Market")]
    pub r#type: Option<OrderType>,
    #[param(example = "Closed")]
    pub status: Option<OrderStatus>,
    #[param(example = "1970-01-01T00:00:00")]
    pub start_time: Option<DateTime>,
    #[param(example = "1970-01-01T00:00:00")]
    pub end_time: Option<DateTime>,
    #[param(example = 0)]
    pub page: Option<u64>,
    #[param(example = 1000)]
    pub page_size: Option<u64>,
}

#[derive(Deserialize, IntoParams)]
pub struct ClientGetOpenRequest {
    #[param(example = 1)]
    pub id: Option<i32>,
    #[param(example = "Test")]
    pub client_order_id: Option<String>,
    #[param(example = 1)]
    pub sub_account_id: Option<i32>,
    #[param(example = "Test")]
    pub sub_account_name: Option<String>,
    #[param(example = 1)]
    pub market_id: Option<i32>,
    #[param(example = "BTC")]
    pub base_currency: Option<String>,
    #[param(example = "USD")]
    pub quote_currency: Option<String>,
    #[param(example = "Buy")]
    pub side: Option<OrderSide>,
}

#[derive(Deserialize, IntoParams)]
pub struct MarketGetRequest {
    #[param(example = "Buy")]
    pub side: Option<OrderSide>,
    #[param(example = "Market")]
    pub r#type: Option<OrderType>,
    #[param(example = "Closed")]
    pub status: Option<OrderStatus>,
    #[param(example = "1970-01-01T00:00:00")]
    pub start_time: Option<DateTime>,
    #[param(example = "1970-01-01T00:00:00")]
    pub end_time: Option<DateTime>,
    #[param(example = 0)]
    pub page: Option<u64>,
    #[param(example = 1000)]
    pub page_size: Option<u64>,
}

#[derive(Deserialize, ToSchema)]
pub struct PostRequest {
    #[schema(example = 1)]
    pub sub_account_id: i32,
    #[schema(example = 100.0)]
    pub size: f32,
    #[schema(example = "Buy")]
    pub side: OrderSide,
    #[schema(example = "Market")]
    pub r#type: OrderType,
    #[schema(example = 50.0)]
    pub price: Option<f32>,
    #[schema(example = "Test")]
    pub client_order_id: Option<String>,
    #[schema(example = 1)]
    pub market_id: Option<i32>,
    #[schema(example = "BTC")]
    pub base_currency: Option<String>,
    #[schema(example = "USD")]
    pub quote_currency: Option<String>,
}

#[derive(Serialize, Deserialize, FromQueryResult, Clone, Debug, ToSchema)]
pub struct Order {
    #[schema(example = 1)]
    pub id: i32,
    #[schema(example = 1)]
    pub sub_account_id: i32,
    #[schema(example = 50.0)]
    pub price: Option<f32>,
    #[schema(example = 100.0)]
    pub size: f32,
    #[schema(example = "Buy")]
    pub side: OrderSide,
    #[schema(example = "Market")]
    pub r#type: OrderType,
    #[schema(example = "1970-01-01T00:00:00")]
    pub open_at: DateTime
}
