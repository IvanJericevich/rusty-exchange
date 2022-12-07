use sea_orm::prelude::*;
use sea_orm::FromQueryResult;
use serde::{Serialize, Deserialize};

pub use crate::entities::{
    clients::Model as Client, markets::Model as Market, sea_orm_active_enums::*,
    sub_accounts::Model as SubAccount,
};

// ----------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, FromQueryResult, Serialize, Deserialize)]
pub struct Order {
    pub id: i32,
    pub price: f32,
    pub size: f32,
    pub filled_size: Option<f32>,
    pub side: OrderSide,
    pub r#type: OrderType,
    pub status: OrderStatus,
    pub open_at: DateTime,
    pub closed_at: Option<DateTime>,
    pub sub_account_id: i32,
    pub market_id: i32,
    pub base_currency: String,
    pub quote_currency: String,
    pub price_increment: f32,
    pub size_increment: f32,
    pub sub_account: String,
    pub client_id: i32,
}

#[derive(Clone, Debug, PartialEq, FromQueryResult, Serialize, Deserialize)]
pub struct Position {
    pub id: i32,
    pub avg_entry_price: f32,
    pub size: f32,
    pub side: OrderSide,
    pub sub_account_id: i32,
    pub market_id: i32,
    pub base_currency: String,
    pub quote_currency: String,
    pub price_increment: f32,
    pub size_increment: f32,
    pub sub_account: String,
    pub client_id: i32,
}
