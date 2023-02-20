//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.2

use sea_orm::entity::prelude::*;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use super::sea_orm_active_enums::OrderSide;
use super::sea_orm_active_enums::OrderType;

// ----------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "fills")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[schema(example = 1)]
    pub id: i32,
    #[schema(example = 50.0)]
    pub price: f32,
    #[schema(example = 100.0)]
    pub size: f32,
    #[schema(example = 100.0)]
    pub quote_size: f32,
    #[schema(example = OrderSide::Buy)]
    pub side: OrderSide,
    #[schema(example = OrderType::Market)]
    pub r#type: OrderType,
    #[schema(example = "1970-01-01T00:00:00")]
    pub created_at: DateTime,
    #[schema(example = 1)]
    pub sub_account_id: i32,
    #[schema(example = 1)]
    pub market_id: i32,
    #[schema(example = 1)]
    pub order_id: i32,
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
        belongs_to = "super::orders::Entity",
        from = "Column::OrderId",
        to = "super::orders::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Orders,
    #[sea_orm(
        belongs_to = "super::sub_accounts::Entity",
        from = "Column::SubAccountId",
        to = "super::sub_accounts::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SubAccounts,
}

// TODO: implement manually so that they all map to same string
impl Related<super::markets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Markets.def()
    }
}

impl Related<super::orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Orders.def()
    }
}

impl Related<super::sub_accounts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SubAccounts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// ----------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, FromQueryResult, Serialize, ToSchema)]
pub struct Response {
    #[schema(example = 50.0)]
    pub price: f32,
    #[schema(example = 100.0)]
    pub size: f32,
    #[schema(example = 100.0)]
    pub quote_size: f32,
    #[schema(example = OrderSide::Buy)]
    pub side: OrderSide,
    #[schema(example = OrderType::Market)]
    pub r#type: OrderType,
    #[schema(example = "1970-01-01T00:00:00")]
    pub created_at: DateTime,
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
    #[schema(example = 1)]
    pub order_id: i32,
}

#[derive(Serialize, Deserialize, Debug)] // TODO: Derive into active model
pub struct Fill {
    pub price: f32,
    pub size: f32,
    pub quote_size: f32,
    pub side: OrderSide,
    pub r#type: OrderType,
    pub created_at: DateTime,
    pub sub_account_id: i32,
    pub market_id: i32,
    pub order_id: i32,
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
    #[param(example = 1)]
    pub order_id: Option<i32>,
    #[param(example = "Buy")]
    pub side: Option<OrderSide>,
    #[param(example = "Market")]
    pub r#type: Option<OrderType>,
    #[param(example = "1970-01-01T00:00:00")]
    pub start_time: Option<DateTime>,
    #[param(example = "1970-01-01T00:00:00")]
    pub end_time: Option<DateTime>,
    #[param(example = 0)]
    pub page: Option<u64>,
    #[param(example = 1000)]
    pub page_size: Option<u64>,
}
