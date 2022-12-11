use sea_orm::{entity::prelude::*, JoinType, QueryOrder, QuerySelect};

use crate::entities::{
    clients, markets, orders, positions,
    sea_orm_active_enums::{OrderSide, OrderStatus, OrderType},
    sub_accounts,
};

use crate::models::{Order, Position};

// ----------------------------------------------------------------------

pub struct Query;

impl Query {
    // Clients
    pub async fn find_client_by_id(db: &DbConn, id: i32) -> Result<clients::Model, DbErr> {
        clients::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "Client with id {id} does not exist."
            )))
    }

    pub async fn find_client_by_email(db: &DbConn, email: String) -> Result<clients::Model, DbErr> {
        clients::Entity::find()
            .filter(clients::Column::Email.eq(email.clone()))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "Client with email {email} does not exist."
            )))
    }

    pub async fn find_clients(
        db: &DbConn,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<Vec<clients::Model>, DbErr> {
        clients::Entity::find()
            .paginate(db, page_size.unwrap_or(1))
            .fetch_page(page.unwrap_or(1) - 1)
            .await
    }
    // ----------------------------------------------------------------------

    // Markets
    pub async fn find_market_by_id(db: &DbConn, id: i32) -> Result<markets::Model, DbErr> {
        markets::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "Market with id {id} does not exist."
            )))
    }

    pub async fn find_market_by_ticker(
        db: &DbConn,
        base_currency: String,
        quote_currency: String,
    ) -> Result<markets::Model, DbErr> {
        markets::Entity::find()
            .filter(markets::Column::BaseCurrency.eq(base_currency.to_uppercase()))
            .filter(markets::Column::QuoteCurrency.eq(quote_currency.to_uppercase()))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "Market with symbol {base_currency}/{quote_currency} does not exist."
            )))
    }

    pub async fn find_markets(
        db: &DbConn,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<Vec<markets::Model>, DbErr> {
        markets::Entity::find()
            .paginate(db, page_size.unwrap_or(1))
            .fetch_page(page.unwrap_or(1) - 1)
            .await
    }
    // ----------------------------------------------------------------------

    // SubAccounts
    pub async fn find_sub_account_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<sub_accounts::Model, DbErr> {
        sub_accounts::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "Sub-account with id {id} does not exist."
            )))
    }

    pub async fn find_sub_accounts_by_client_id(
        db: &DbConn,
        id: i32,
    ) -> Result<Vec<sub_accounts::Model>, DbErr> {
        let client: Option<clients::Model> = clients::Entity::find_by_id(id).one(db).await?;
        match client {
            Some(client) => client.find_related(sub_accounts::Entity).all(db).await,
            None => Err(DbErr::RecordNotFound("Client does not exist.".to_owned())),
        }
    }

    pub async fn find_sub_accounts(
        db: &DbConn,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<Vec<sub_accounts::Model>, DbErr> {
        sub_accounts::Entity::find()
            .paginate(db, page_size.unwrap_or(1))
            .fetch_page(page.unwrap_or(1) - 1)
            .await
    }
    // ----------------------------------------------------------------------

    // Orders
    #[allow(clippy::too_many_arguments)]
    pub async fn find_orders(
        db: &DbConn,
        client_order_id: Option<String>,
        side: Option<OrderSide>,
        r#type: Option<OrderType>,
        sub_account: Option<String>,
        client_id: Option<i32>,
        status: Option<OrderStatus>,
        base_currency: Option<String>,
        quote_currency: Option<String>,
        start_time: Option<DateTime>,
        end_time: Option<DateTime>,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<Vec<Order>, DbErr> {
        let mut query = orders::Entity::find()
            .join(JoinType::InnerJoin, orders::Relation::SubAccounts.def())
            .column_as(sub_accounts::Column::Name, "sub_account")
            .column(sub_accounts::Column::ClientId)
            .join(JoinType::InnerJoin, orders::Relation::Markets.def())
            .column(markets::Column::BaseCurrency)
            .column(markets::Column::QuoteCurrency)
            .column(markets::Column::PriceIncrement)
            .column(markets::Column::SizeIncrement)
            .order_by_asc(match status {
                Some(OrderStatus::Open) => orders::Column::OpenAt,
                _ => orders::Column::ClosedAt,
            });
        if let Some(client_order_id) = client_order_id {
            query = query.filter(orders::Column::ClientOrderId.eq(client_order_id));
        }
        if let Some(side) = side {
            query = query.filter(orders::Column::Side.eq(side));
        }
        if let Some(r#type) = r#type {
            query = query.filter(orders::Column::Type.eq(r#type));
        }
        if let Some(sub_account) = sub_account {
            query = query.filter(sub_accounts::Column::Name.eq(sub_account));
        }
        if let Some(client_id) = client_id {
            query = query.filter(sub_accounts::Column::ClientId.eq(client_id));
        }
        if let Some(status) = status.clone() {
            query = query.filter(orders::Column::Status.eq(status));
        }
        if let Some(base_currency) = base_currency {
            query = query.filter(markets::Column::BaseCurrency.eq(base_currency.to_uppercase()));
        }
        if let Some(quote_currency) = quote_currency {
            query = query.filter(markets::Column::QuoteCurrency.eq(quote_currency.to_uppercase()));
        }
        if let Some(start_time) = start_time {
            query = query.filter(match status {
                Some(OrderStatus::Open) => orders::Column::OpenAt.gt(start_time),
                _ => orders::Column::ClosedAt.gt(start_time),
            });
        }
        if let Some(end_time) = end_time {
            query = query.filter(match status {
                Some(OrderStatus::Open) => orders::Column::OpenAt.lt(end_time),
                _ => orders::Column::ClosedAt.lt(end_time),
            });
        }

        query
            .into_model::<Order>()
            .paginate(db, page_size.unwrap_or(1))
            .fetch_page(page.unwrap_or(1) - 1)
            .await
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
    ) -> Result<Vec<Position>, DbErr> {
        let mut query = positions::Entity::find()
            .join(JoinType::InnerJoin, positions::Relation::SubAccounts.def())
            .column_as(sub_accounts::Column::Name, "sub_account")
            .column(sub_accounts::Column::ClientId)
            .join(JoinType::InnerJoin, positions::Relation::Markets.def())
            .column(markets::Column::BaseCurrency)
            .column(markets::Column::QuoteCurrency)
            .column(markets::Column::PriceIncrement)
            .column(markets::Column::SizeIncrement);

        if let Some(sub_account) = sub_account {
            query = query.filter(sub_accounts::Column::Name.eq(sub_account));
        }
        if let Some(base_currency) = base_currency {
            query = query.filter(markets::Column::BaseCurrency.eq(base_currency.to_uppercase()));
        }
        if let Some(quote_currency) = quote_currency {
            query = query.filter(markets::Column::QuoteCurrency.eq(quote_currency.to_uppercase()));
        }

        query
            .into_model::<Position>()
            .paginate(db, page_size.unwrap_or(1))
            .fetch_page(page.unwrap_or(1) - 1)
            .await
    }
    // ----------------------------------------------------------------------
}
