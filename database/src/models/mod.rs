use sea_orm::prelude::*;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

pub use crate::entities::{
    clients::ActiveModel as ActiveClientModel, clients::Model as ClientModel,
    markets::ActiveModel as ActiveMarketModel, markets::Model as MarketModel,
    orders::ActiveModel as ActiveOrderModel, orders::Model as OrderModel, sea_orm_active_enums::*,
    sub_accounts::ActiveModel as ActiveSubAccountModel, sub_accounts::Model as SubAccountModel,
};

// ----------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, FromQueryResult, Serialize, Deserialize)]
pub struct Order {
    pub id: i32,
    pub client_order_id: Option<String>,
    pub price: f32,
    pub size: f32,
    pub quote_size: f32,
    pub filled_size: Option<f32>,
    pub side: OrderSide,
    pub r#type: OrderType,
    pub status: OrderStatus,
    pub open_at: DateTime,
    pub closed_at: Option<DateTime>,
    pub base_currency: String,
    pub quote_currency: String,
    pub price_increment: f32,
    pub size_increment: f32,
    pub sub_account: String,
}

#[derive(Clone, Debug, PartialEq, FromQueryResult, Serialize, Deserialize)]
pub struct Position {
    // pub id: i32,
    pub avg_entry_price: f32,
    pub size: f32,
    pub side: OrderSide,
    // pub sub_account_id: i32,
    // pub market_id: i32,
    pub base_currency: String,
    pub quote_currency: String,
    pub price_increment: f32,
    pub size_increment: f32,
    pub sub_account: String,
    // pub client_id: i32,
}
