// TODO: Streams
use sea_orm::{entity::prelude::*, FromQueryResult, JoinType, QuerySelect, QueryOrder};

use crate::{
    entities::{
        markets, markets::Entity as Markets,
        sub_accounts, sub_accounts::Entity as SubAccounts,
        orders, orders::Entity as Orders,
        positions, positions::Entity as Positions,
        clients, clients::Entity as Clients
    },
    sea_orm_active_enums::{OrderSide, OrderType, OrderStatus}
};

// ----------------------------------------------------------------------

#[derive(Debug, FromQueryResult, PartialEq)]
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

// ----------------------------------------------------------------------

pub struct Query;

impl Query {
    // Clients
    pub async fn find_client_by_id(
        db: &DbConn,
        id: i32
    ) -> Result<Option<clients::Model>, DbErr> {
        Clients::find_by_id(id).one(db).await
    }

    pub async fn find_client_by_email(
        db: &DbConn,
        email: String
    ) -> Result<Option<clients::Model>, DbErr> {
        Clients::find()
            .filter(clients::Column::Email.eq(email))
            .one(db).await
    }
    // ----------------------------------------------------------------------

    // Markets
    pub async fn find_market_by_id(
        db: &DbConn,
        id: i32
    ) -> Result<Option<markets::Model>, DbErr> {
        Markets::find_by_id(id).one(db).await
    }

    pub async fn find_market_by_ticker(
        db: &DbConn,
        base_currency: String,
        quote_currency: String
    ) -> Result<Option<markets::Model>, DbErr> {
        Markets::find()
            .filter(markets::Column::BaseCurrency.eq(base_currency.to_uppercase()))
            .filter(markets::Column::QuoteCurrency.eq(quote_currency.to_uppercase()))
            .one(db).await
    }
    // ----------------------------------------------------------------------

    // SubAccounts
    pub async fn find_sub_account_by_id(
        db: &DbConn,
        id: i32
    ) -> Result<Option<sub_accounts::Model>, DbErr> {
        SubAccounts::find_by_id(id).one(db).await
    }

    pub async fn find_sub_account_by_client_id(
        db: &DbConn,
        id: i32
    ) -> Result<Vec<(clients::Model, Vec<sub_accounts::Model>)>, DbErr> { // One-to-many relationship
        Clients::find_by_id(id)
            .find_with_related(SubAccounts)
            .all(db).await
    }
    // ----------------------------------------------------------------------

    // Orders
    pub async fn find_orders(
        db: &DbConn,
        side: Option<OrderSide>,
        r#type: Option<OrderType>,
        sub_account: Option<String>,
        client_id: Option<i32>,
        status: Option<OrderStatus>,
        base_currency: Option<String>,
        quote_currency: Option<String>,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<Vec<Order>, DbErr> {
        let mut query = Orders::find()
            .join(JoinType::InnerJoin, orders::Relation::SubAccounts.def())
            .column_as(sub_accounts::Column::Name, "sub_account")
            .column(sub_accounts::Column::ClientId)
            .join(JoinType::InnerJoin, orders::Relation::Markets.def())
            .column(markets::Column::BaseCurrency)
            .column(markets::Column::QuoteCurrency)
            .column(markets::Column::PriceIncrement)
            .column(markets::Column::SizeIncrement)
            .order_by_asc(
                match status {
                    Some(OrderStatus::Open) => orders::Column::OpenAt,
                    _ => orders::Column::ClosedAt,
                }
            );

        if let Some(x) = side {
            query = query.filter(orders::Column::Side.eq(x));
        }
        if let Some(x) = r#type {
            query = query.filter(orders::Column::Type.eq(x));
        }
        if let Some(x) = sub_account {
            query = query.filter(sub_accounts::Column::Name.eq(x));
        }
        if let Some(x) = client_id {
            query = query.filter(sub_accounts::Column::ClientId.eq(x));
        }
        if let Some(x) = status.clone() {
            query = query.filter(orders::Column::Status.eq(x));
        }
        if let Some(x) = base_currency {
            query = query.filter(markets::Column::BaseCurrency.eq(x.to_uppercase()));
        }
        if let Some(x) = quote_currency {
            query = query.filter(markets::Column::QuoteCurrency.eq(x.to_uppercase()));
        }
            
        query.into_model::<Order>()
            .paginate(db, page_size.unwrap_or_default())
            .fetch_page(page.unwrap_or(1) - 1).await
    }
    // ----------------------------------------------------------------------

    // Positions
    pub async fn find_positions(
        db: &DbConn,
        sub_account: Option<String>,
        base_currency: Option<String>,
        quote_currency: Option<String>,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<Vec<positions::Model>, DbErr> {
        let mut query = Positions::find()
            .join(JoinType::InnerJoin, positions::Relation::SubAccounts.def())
            .column_as(sub_accounts::Column::Name, "sub_account")
            .column(sub_accounts::Column::ClientId)
            .join(JoinType::InnerJoin, positions::Relation::Markets.def())
            .column(markets::Column::BaseCurrency)
            .column(markets::Column::QuoteCurrency)
            .column(markets::Column::PriceIncrement)
            .column(markets::Column::SizeIncrement);

        if let Some(x) = sub_account {
            query = query.filter(sub_accounts::Column::Name.eq(x));
        }
        if let Some(x) = base_currency {
            query = query.filter(markets::Column::BaseCurrency.eq(x.to_uppercase()));
        }
        if let Some(x) = quote_currency {
            query = query.filter(markets::Column::QuoteCurrency.eq(x.to_uppercase()));
        }

        query.paginate(db, page_size.unwrap_or_default())
            .fetch_page(page.unwrap_or(1) - 1).await
    }
    // ----------------------------------------------------------------------
}
