use std::cmp::min;
use sea_orm::prelude::*;
use sea_orm::*;
use sea_orm_migration::sea_query::Query as SeaQuery;

use crate::entities::{clients, fills, markets, orders, positions, sea_orm_active_enums::{OrderSide, OrderStatus, OrderType, SubAccountStatus}, sub_accounts};

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
            .paginate(db, min(page_size.unwrap_or(1), 1000))
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
                "Market with base currency {base_currency} and quote currency {quote_currency} does not exist."
            )))
    }

    pub async fn find_markets(
        db: &DbConn,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<Vec<markets::Model>, DbErr> {
        markets::Entity::find()
            .paginate(db, min(page_size.unwrap_or(1), 1000))
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
        if let Some(client) = clients::Entity::find_by_id(id).one(db).await? {
            client
                .find_related(sub_accounts::Entity)
                .filter(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                .all(db)
                .await
        } else {
            Err(DbErr::RecordNotFound(format!(
                "Client with id {id} does not exist."
            )))
        }
    }

    pub async fn find_sub_accounts(
        db: &DbConn,
        status: Option<SubAccountStatus>,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<Vec<sub_accounts::Model>, DbErr> {
        let mut query = sub_accounts::Entity::find();
        if let Some(status) = status {
            query = query.filter(sub_accounts::Column::Status.eq(status));
        }

        query
            .paginate(db, min(page_size.unwrap_or(1), 1000))
            .fetch_page(page.unwrap_or(1) - 1)
            .await
    }
    // ----------------------------------------------------------------------

    // Orders
    pub async fn find_client_related_open_order(
        db: &DbConn,
        client_id: i32,
        id: Option<i32>,
        client_order_id: Option<String>,
        sub_account_id: Option<i32>,
        sub_account_name: Option<String>,
        market_id: Option<i32>,
        base_currency: Option<String>,
        quote_currency: Option<String>,
        side: Option<OrderSide>,
    ) -> Result<orders::Model, DbErr> {
        if let Some(client) = clients::Entity::find_by_id(client_id).one(db).await? {
            let mut conditions = Condition::all().add(
                orders::Column::SubAccountId.in_subquery(
                    if let Some(sub_account_id) = sub_account_id {
                        if let Some(sub_account) = sub_accounts::Entity::find_by_id(sub_account_id)
                            .filter(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                            .one(db)
                            .await?
                        {
                            SeaQuery::select()
                                .column(sub_accounts::Column::Id)
                                .from(sub_accounts::Entity)
                                .and_where(sub_accounts::Column::Id.eq(sub_account.id))
                                .and_where(sub_accounts::Column::ClientId.eq(client.id))
                                .and_where(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                                .to_owned()
                        } else {
                            return Err(DbErr::RecordNotFound(format!(
                                "Sub-account with id {sub_account_id} does not exist."
                            )))
                        }
                    } else if let Some(sub_account_name) = sub_account_name {
                        if let Some(sub_account) = sub_accounts::Entity::find()
                            .filter(sub_accounts::Column::Name.eq(sub_account_name.clone()))
                            .filter(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                            .one(db)
                            .await?
                        {
                            SeaQuery::select()
                                .column(sub_accounts::Column::Id)
                                .from(sub_accounts::Entity)
                                .and_where(sub_accounts::Column::Name.eq(sub_account.name))
                                .and_where(sub_accounts::Column::ClientId.eq(client.id))
                                .and_where(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                                .to_owned()
                        } else {
                            return Err(DbErr::RecordNotFound(format!(
                                "Sub-account with name {sub_account_name} does not exist."
                            )))
                        }
                    } else {
                        SeaQuery::select()
                            .column(sub_accounts::Column::Id)
                            .from(sub_accounts::Entity)
                            .and_where(sub_accounts::Column::ClientId.eq(client.id))
                            .and_where(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                            .to_owned()
                    }
                ),
            );

            if let Some(market_id) = market_id {
                if let Some(market) = markets::Entity::find_by_id(market_id)
                    .one(db)
                    .await?
                {
                    conditions = conditions.add(
                        orders::Column::MarketId.in_subquery(
                            SeaQuery::select()
                                .column(markets::Column::Id)
                                .from(markets::Entity)
                                .and_where(markets::Column::Id.eq(market.id))
                                .to_owned(),
                        ),
                    );
                } else {
                    return Err(DbErr::RecordNotFound(format!(
                        "Market with id {market_id} does not exist."
                    )))
                }
            } else {
                match (base_currency, quote_currency) {
                    (Some(base_currency), Some(quote_currency)) => {
                        if let Some(market) = markets::Entity::find()
                            .filter(markets::Column::BaseCurrency.eq(base_currency.to_uppercase()))
                            .filter(markets::Column::QuoteCurrency.eq(quote_currency.to_uppercase()))
                            .one(db)
                            .await?
                        {
                            conditions = conditions.add(
                                orders::Column::MarketId.in_subquery(
                                    SeaQuery::select()
                                        .column(markets::Column::Id)
                                        .from(markets::Entity)
                                        .and_where(markets::Column::BaseCurrency.eq(market.base_currency))
                                        .and_where(markets::Column::QuoteCurrency.eq(market.quote_currency))
                                        .to_owned(),
                                ),
                            );
                        } else {
                            return Err(DbErr::RecordNotFound(format!(
                                "Market with base currency {base_currency} and quote currency {quote_currency} does not exist."
                            )))
                        }
                    },
                    (Some(base_currency), None) => {
                        if let Some(market) = markets::Entity::find()
                            .filter(markets::Column::BaseCurrency.eq(base_currency.to_uppercase()))
                            .one(db)
                            .await?
                        {
                            conditions = conditions.add(
                                orders::Column::MarketId.in_subquery(
                                    SeaQuery::select()
                                        .column(markets::Column::Id)
                                        .from(markets::Entity)
                                        .and_where(markets::Column::BaseCurrency.eq(market.base_currency))
                                        .to_owned(),
                                ),
                            );
                        } else {
                            return Err(DbErr::RecordNotFound(format!(
                                "Market with base currency {base_currency} does not exist."
                            )))
                        }
                    },
                    (None, Some(quote_currency)) => {
                        if let Some(market) = markets::Entity::find()
                            .filter(markets::Column::QuoteCurrency.eq(quote_currency.to_uppercase()))
                            .one(db)
                            .await?
                        {
                            conditions = conditions.add(
                                orders::Column::MarketId.in_subquery(
                                    SeaQuery::select()
                                        .column(markets::Column::Id)
                                        .from(markets::Entity)
                                        .and_where(markets::Column::QuoteCurrency.eq(market.quote_currency))
                                        .to_owned(),
                                ),
                            );
                        } else {
                            return Err(DbErr::RecordNotFound(format!(
                                "Market with quote currency {quote_currency} does not exist."
                            )))
                        }
                    },
                    (None, None) => {},
                }
            }

            let mut query = if let Some(id) = id {
                orders::Entity::find_by_id(id)
                    .filter(conditions)
                    .filter(orders::Column::Status.eq(OrderStatus::Open))
            } else if let Some(client_order_id) = client_order_id {
                orders::Entity::find()
                    .filter(conditions)
                    .filter(orders::Column::ClientOrderId.contains(client_order_id.clone().as_str()))
                    .filter(orders::Column::Status.eq(OrderStatus::Open))
            } else {
                orders::Entity::find()
                    .filter(conditions)
                    .filter(orders::Column::Status.eq(OrderStatus::Open))
            };
            if let Some(side) = side {
                query = query.filter(orders::Column::Side.eq(side));
            }
            query.one(db).await?.ok_or(
                DbErr::RecordNotFound(format!(
                    "Order does not exist."
                ))
            )
        } else {
            Err(DbErr::RecordNotFound(format!(
                "Client with id {client_id} does not exist."
            )))
        }
    }

    pub async fn find_client_related_orders(
        db: &DbConn,
        client_id: i32,
        sub_account_id: Option<i32>,
        sub_account_name: Option<String>,
        market_id: Option<i32>,
        base_currency: Option<String>,
        quote_currency: Option<String>,
        client_order_id: Option<String>,
        side: Option<OrderSide>,
        r#type: Option<OrderType>,
        status: Option<OrderStatus>,
        start_time: Option<DateTime>,
        end_time: Option<DateTime>,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<Vec<orders::Response>, DbErr> {
        if let Some(client) = clients::Entity::find_by_id(client_id).one(db).await? {
            let mut conditions = Condition::all().add(
                orders::Column::SubAccountId.in_subquery(
                    if let Some(sub_account_id) = sub_account_id {
                        if let Some(sub_account) = sub_accounts::Entity::find_by_id(sub_account_id)
                            .filter(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                            .one(db)
                            .await?
                        {
                            SeaQuery::select()
                                .column(sub_accounts::Column::Id)
                                .from(sub_accounts::Entity)
                                .and_where(sub_accounts::Column::Id.eq(sub_account.id))
                                .and_where(sub_accounts::Column::ClientId.eq(client.id))
                                .and_where(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                                .to_owned()
                        } else {
                            return Err(DbErr::RecordNotFound(format!(
                                "Sub-account with id {sub_account_id} does not exist."
                            )))
                        }
                    } else if let Some(sub_account_name) = sub_account_name {
                        if let Some(sub_account) = sub_accounts::Entity::find()
                            .filter(sub_accounts::Column::Name.eq(sub_account_name.clone()))
                            .filter(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                            .one(db)
                            .await?
                        {
                            SeaQuery::select()
                                .column(sub_accounts::Column::Id)
                                .from(sub_accounts::Entity)
                                .and_where(sub_accounts::Column::Name.eq(sub_account.name))
                                .and_where(sub_accounts::Column::ClientId.eq(client.id))
                                .and_where(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                                .to_owned()
                        } else {
                            return Err(DbErr::RecordNotFound(format!(
                                "Sub-account with name {sub_account_name} does not exist."
                            )))
                        }
                    } else {
                        SeaQuery::select()
                            .column(sub_accounts::Column::Id)
                            .from(sub_accounts::Entity)
                            .and_where(sub_accounts::Column::ClientId.eq(client.id))
                            .and_where(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                            .to_owned()
                    }
                ),
            );

            if let Some(market_id) = market_id {
                if let Some(market) = markets::Entity::find_by_id(market_id)
                    .one(db)
                    .await?
                {
                    conditions = conditions.add(
                        orders::Column::MarketId.in_subquery(
                            SeaQuery::select()
                                .column(markets::Column::Id)
                                .from(markets::Entity)
                                .and_where(markets::Column::Id.eq(market.id))
                                .to_owned(),
                        ),
                    );
                } else {
                    return Err(DbErr::RecordNotFound(format!(
                        "Market with id {market_id} does not exist."
                    )))
                }
            } else {
                match (base_currency, quote_currency) {
                    (Some(base_currency), Some(quote_currency)) => {
                        if let Some(market) = markets::Entity::find()
                            .filter(markets::Column::BaseCurrency.eq(base_currency.to_uppercase()))
                            .filter(markets::Column::QuoteCurrency.eq(quote_currency.to_uppercase()))
                            .one(db)
                            .await?
                        {
                            conditions = conditions.add(
                                orders::Column::MarketId.in_subquery(
                                    SeaQuery::select()
                                        .column(markets::Column::Id)
                                        .from(markets::Entity)
                                        .and_where(markets::Column::BaseCurrency.eq(market.base_currency))
                                        .and_where(markets::Column::QuoteCurrency.eq(market.quote_currency))
                                        .to_owned(),
                                ),
                            );
                        } else {
                            return Err(DbErr::RecordNotFound(format!(
                                "Market with base currency {base_currency} and quote currency {quote_currency} does not exist."
                            )))
                        }
                    },
                    (Some(base_currency), None) => {
                        if let Some(market) = markets::Entity::find()
                            .filter(markets::Column::BaseCurrency.eq(base_currency.to_uppercase()))
                            .one(db)
                            .await?
                        {
                            conditions = conditions.add(
                                orders::Column::MarketId.in_subquery(
                                    SeaQuery::select()
                                        .column(markets::Column::Id)
                                        .from(markets::Entity)
                                        .and_where(markets::Column::BaseCurrency.eq(market.base_currency))
                                        .to_owned(),
                                ),
                            );
                        } else {
                            return Err(DbErr::RecordNotFound(format!(
                                "Market with base currency {base_currency} does not exist."
                            )))
                        }
                    },
                    (None, Some(quote_currency)) => {
                        if let Some(market) = markets::Entity::find()
                            .filter(markets::Column::QuoteCurrency.eq(quote_currency.to_uppercase()))
                            .one(db)
                            .await?
                        {
                            conditions = conditions.add(
                                orders::Column::MarketId.in_subquery(
                                    SeaQuery::select()
                                        .column(markets::Column::Id)
                                        .from(markets::Entity)
                                        .and_where(markets::Column::QuoteCurrency.eq(market.quote_currency))
                                        .to_owned(),
                                ),
                            );
                        } else {
                            return Err(DbErr::RecordNotFound(format!(
                                "Market with quote currency {quote_currency} does not exist."
                            )))
                        }
                    },
                    (None, None) => {},
                }
            }

            let mut query = orders::Entity::find().filter(conditions);
            if let Some(client_order_id) = client_order_id {
                query = query.filter(orders::Column::ClientOrderId.like(client_order_id.as_str()));
            }
            if let Some(side) = side {
                query = query.filter(orders::Column::Side.eq(side));
            }
            if let Some(r#type) = r#type {
                query = query.filter(orders::Column::Type.eq(r#type));
            }
            if let Some(status) = status.clone() {
                query = query.filter(orders::Column::Status.eq(status));
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
                .inner_join(sub_accounts::Entity)
                .column_as(sub_accounts::Column::Name, "sub_account")
                .inner_join(markets::Entity)
                .column(markets::Column::BaseCurrency)
                .column(markets::Column::QuoteCurrency)
                .column(markets::Column::PriceIncrement)
                .column(markets::Column::SizeIncrement)
                .order_by_asc(match status {
                    Some(OrderStatus::Open) => orders::Column::OpenAt,
                    _ => orders::Column::ClosedAt,
                })
                .into_model::<orders::Response>()
                .paginate(db, min(page_size.unwrap_or(1), 1000))
                .fetch_page(page.unwrap_or(1) - 1)
                .await
        } else {
            Err(DbErr::RecordNotFound(format!(
                "Client with id {client_id} does not exist."
            )))
        }
    }

    pub async fn find_market_related_orders(
        db: &DbConn,
        id: i32,
        side: Option<OrderSide>,
        r#type: Option<OrderType>,
        status: Option<OrderStatus>,
        start_time: Option<DateTime>,
        end_time: Option<DateTime>,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<Vec<orders::Response>, DbErr> {
        let mut query = orders::Entity::find().filter(
            Condition::all().add(
                if let Some(market) = markets::Entity::find_by_id(id)
                    .one(db)
                    .await?
                {
                    orders::Column::MarketId.in_subquery(
                        SeaQuery::select()
                            .column(markets::Column::Id)
                            .from(markets::Entity)
                            .and_where(markets::Column::Id.eq(market.id))
                            .to_owned(),
                    )
                } else {
                    return Err(DbErr::RecordNotFound(format!(
                        "Market with id {id} does not exist."
                    )))
                },
            ),
        );
        if let Some(side) = side {
            query = query.filter(orders::Column::Side.eq(side));
        }
        if let Some(r#type) = r#type {
            query = query.filter(orders::Column::Type.eq(r#type));
        }
        if let Some(status) = status.clone() {
            query = query.filter(orders::Column::Status.eq(status));
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
            .inner_join(sub_accounts::Entity)
            .column_as(sub_accounts::Column::Name, "sub_account")
            .inner_join(markets::Entity)
            .column(markets::Column::BaseCurrency)
            .column(markets::Column::QuoteCurrency)
            .column(markets::Column::PriceIncrement)
            .column(markets::Column::SizeIncrement)
            .order_by_asc(match status {
                Some(OrderStatus::Open) => orders::Column::OpenAt,
                _ => orders::Column::ClosedAt,
            })
            .into_model::<orders::Response>()
            .paginate(db, min(page_size.unwrap_or(1), 1000))
            .fetch_page(page.unwrap_or(1) - 1)
            .await
    }
    // ----------------------------------------------------------------------

    // Fills
    pub async fn find_client_related_fills(
        db: &DbConn,
        client_id: i32,
        sub_account_id: Option<i32>,
        sub_account_name: Option<String>,
        market_id: Option<i32>,
        base_currency: Option<String>,
        quote_currency: Option<String>,
        order_id: Option<i32>,
        side: Option<OrderSide>,
        r#type: Option<OrderType>,
        start_time: Option<DateTime>,
        end_time: Option<DateTime>,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<Vec<fills::Response>, DbErr> {
        if let Some(client) = clients::Entity::find_by_id(client_id).one(db).await? {
            let mut conditions = Condition::all().add(
                fills::Column::SubAccountId.in_subquery(
                    if let Some(sub_account_id) = sub_account_id {
                        if let Some(sub_account) = sub_accounts::Entity::find_by_id(sub_account_id)
                            .filter(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                            .one(db)
                            .await?
                        {
                            SeaQuery::select()
                                .column(sub_accounts::Column::Id)
                                .from(sub_accounts::Entity)
                                .and_where(sub_accounts::Column::Id.eq(sub_account.id))
                                .and_where(sub_accounts::Column::ClientId.eq(client.id))
                                .and_where(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                                .to_owned()
                        } else {
                            return Err(DbErr::RecordNotFound(format!(
                                "Sub-account with id {sub_account_id} does not exist."
                            )))
                        }
                    } else if let Some(sub_account_name) = sub_account_name {
                        if let Some(sub_account) = sub_accounts::Entity::find()
                            .filter(sub_accounts::Column::Name.eq(sub_account_name.clone()))
                            .filter(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                            .one(db)
                            .await?
                        {
                            SeaQuery::select()
                                .column(sub_accounts::Column::Id)
                                .from(sub_accounts::Entity)
                                .and_where(sub_accounts::Column::Name.eq(sub_account.name))
                                .and_where(sub_accounts::Column::ClientId.eq(client.id))
                                .and_where(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                                .to_owned()
                        } else {
                            return Err(DbErr::RecordNotFound(format!(
                                "Sub-account with name {sub_account_name} does not exist."
                            )))
                        }
                    } else {
                        SeaQuery::select()
                            .column(sub_accounts::Column::Id)
                            .from(sub_accounts::Entity)
                            .and_where(sub_accounts::Column::ClientId.eq(client.id))
                            .and_where(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                            .to_owned()
                    }
                ),
            );

            if let Some(market_id) = market_id {
                if let Some(market) = markets::Entity::find_by_id(market_id)
                    .one(db)
                    .await?
                {
                    conditions = conditions.add(
                        fills::Column::MarketId.in_subquery(
                            SeaQuery::select()
                                .column(markets::Column::Id)
                                .from(markets::Entity)
                                .and_where(markets::Column::Id.eq(market.id))
                                .to_owned(),
                        ),
                    );
                } else {
                    return Err(DbErr::RecordNotFound(format!(
                        "Market with id {market_id} does not exist."
                    )))
                }
            } else {
                match (base_currency, quote_currency) {
                    (Some(base_currency), Some(quote_currency)) => {
                        if let Some(market) = markets::Entity::find()
                            .filter(markets::Column::BaseCurrency.eq(base_currency.to_uppercase()))
                            .filter(markets::Column::QuoteCurrency.eq(quote_currency.to_uppercase()))
                            .one(db)
                            .await?
                        {
                            conditions = conditions.add(
                                fills::Column::MarketId.in_subquery(
                                    SeaQuery::select()
                                        .column(markets::Column::Id)
                                        .from(markets::Entity)
                                        .and_where(markets::Column::BaseCurrency.eq(market.base_currency))
                                        .and_where(markets::Column::QuoteCurrency.eq(market.quote_currency))
                                        .to_owned(),
                                ),
                            );
                        } else {
                            return Err(DbErr::RecordNotFound(format!(
                                "Market with base currency {base_currency} and quote currency {quote_currency} does not exist."
                            )))
                        }
                    },
                    (Some(base_currency), None) => {
                        if let Some(market) = markets::Entity::find()
                            .filter(markets::Column::BaseCurrency.eq(base_currency.to_uppercase()))
                            .one(db)
                            .await?
                        {
                            conditions = conditions.add(
                                fills::Column::MarketId.in_subquery(
                                    SeaQuery::select()
                                        .column(markets::Column::Id)
                                        .from(markets::Entity)
                                        .and_where(markets::Column::BaseCurrency.eq(market.base_currency))
                                        .to_owned(),
                                ),
                            );
                        } else {
                            return Err(DbErr::RecordNotFound(format!(
                                "Market with base currency {base_currency} does not exist."
                            )))
                        }
                    },
                    (None, Some(quote_currency)) => {
                        if let Some(market) = markets::Entity::find()
                            .filter(markets::Column::QuoteCurrency.eq(quote_currency.to_uppercase()))
                            .one(db)
                            .await?
                        {
                            conditions = conditions.add(
                                fills::Column::MarketId.in_subquery(
                                    SeaQuery::select()
                                        .column(markets::Column::Id)
                                        .from(markets::Entity)
                                        .and_where(markets::Column::QuoteCurrency.eq(market.quote_currency))
                                        .to_owned(),
                                ),
                            );
                        } else {
                            return Err(DbErr::RecordNotFound(format!(
                                "Market with quote currency {quote_currency} does not exist."
                            )))
                        }
                    },
                    (None, None) => {},
                }
            }

            if let Some(order_id) = order_id {
                conditions = conditions.add(
                    fills::Column::OrderId.in_subquery(
                        SeaQuery::select()
                            .column(orders::Column::Id)
                            .from(orders::Entity)
                            .and_where(orders::Column::Id.eq(order_id))
                            .to_owned(),
                    ),
                );
            }

            let mut query = fills::Entity::find().filter(conditions);
            if let Some(side) = side {
                query = query.filter(fills::Column::Side.eq(side));
            }
            if let Some(r#type) = r#type {
                query = query.filter(fills::Column::Type.eq(r#type));
            }
            if let Some(start_time) = start_time {
                query = query.filter(fills::Column::CreatedAt.gt(start_time));
            }
            if let Some(end_time) = end_time {
                query = query.filter(fills::Column::CreatedAt.lt(end_time));
            }

            query
                .inner_join(sub_accounts::Entity)
                .column_as(sub_accounts::Column::Name, "sub_account")
                .inner_join(markets::Entity)
                .column(markets::Column::BaseCurrency)
                .column(markets::Column::QuoteCurrency)
                .column(markets::Column::PriceIncrement)
                .column(markets::Column::SizeIncrement)
                .order_by_asc(fills::Column::CreatedAt)
                .into_model::<fills::Response>()
                .paginate(db, min(page_size.unwrap_or(1), 1000))
                .fetch_page(page.unwrap_or(1) - 1)
                .await
        } else {
            Err(DbErr::RecordNotFound(format!(
                "Client with id {client_id} does not exist."
            )))
        }
    }
    // ----------------------------------------------------------------------

    // Positions
    pub async fn find_client_related_positions(
        db: &DbConn,
        client_id: i32,
        sub_account_id: Option<i32>,
        sub_account_name: Option<String>,
        market_id: Option<i32>,
        base_currency: Option<String>,
        quote_currency: Option<String>,
        side: Option<OrderSide>,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<Vec<positions::Response>, DbErr> {
        if let Some(client) = clients::Entity::find_by_id(client_id).one(db).await? {
            let mut conditions = Condition::all().add(
                positions::Column::SubAccountId.in_subquery(
                    if let Some(sub_account_id) = sub_account_id {
                        if let Some(sub_account) = sub_accounts::Entity::find_by_id(sub_account_id)
                            .filter(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                            .one(db)
                            .await?
                        {
                            SeaQuery::select()
                                .column(sub_accounts::Column::Id)
                                .from(sub_accounts::Entity)
                                .and_where(sub_accounts::Column::Id.eq(sub_account.id))
                                .and_where(sub_accounts::Column::ClientId.eq(client.id))
                                .and_where(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                                .to_owned()
                        } else {
                            return Err(DbErr::RecordNotFound(format!(
                                "Sub-account with id {sub_account_id} does not exist."
                            )))
                        }
                    } else if let Some(sub_account_name) = sub_account_name {
                        if let Some(sub_account) = sub_accounts::Entity::find()
                            .filter(sub_accounts::Column::Name.eq(sub_account_name.clone()))
                            .filter(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                            .one(db)
                            .await?
                        {
                            SeaQuery::select()
                                .column(sub_accounts::Column::Id)
                                .from(sub_accounts::Entity)
                                .and_where(sub_accounts::Column::Name.eq(sub_account.name))
                                .and_where(sub_accounts::Column::ClientId.eq(client.id))
                                .and_where(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                                .to_owned()
                        } else {
                            return Err(DbErr::RecordNotFound(format!(
                                "Sub-account with name {sub_account_name} does not exist."
                            )))
                        }
                    } else {
                        SeaQuery::select()
                            .column(sub_accounts::Column::Id)
                            .from(sub_accounts::Entity)
                            .and_where(sub_accounts::Column::ClientId.eq(client.id))
                            .and_where(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                            .to_owned()
                    }
                ),
            );

            if let Some(market_id) = market_id {
                if let Some(market) = markets::Entity::find_by_id(market_id)
                    .filter(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                    .one(db)
                    .await?
                {
                    conditions = conditions.add(
                        orders::Column::MarketId.in_subquery(
                            SeaQuery::select()
                                .column(markets::Column::Id)
                                .from(markets::Entity)
                                .and_where(markets::Column::Id.eq(market.id))
                                .to_owned(),
                        ),
                    );
                } else {
                    return Err(DbErr::RecordNotFound(format!(
                        "Market with id {market_id} does not exist."
                    )))
                }
            } else {
                match (base_currency, quote_currency) {
                    (Some(base_currency), Some(quote_currency)) => {
                        if let Some(market) = markets::Entity::find()
                            .filter(markets::Column::BaseCurrency.eq(base_currency.to_uppercase()))
                            .filter(markets::Column::QuoteCurrency.eq(quote_currency.to_uppercase()))
                            .one(db)
                            .await?
                        {
                            conditions = conditions.add(
                                orders::Column::MarketId.in_subquery(
                                    SeaQuery::select()
                                        .column(markets::Column::Id)
                                        .from(markets::Entity)
                                        .and_where(markets::Column::BaseCurrency.eq(market.base_currency))
                                        .and_where(markets::Column::QuoteCurrency.eq(market.quote_currency))
                                        .to_owned(),
                                ),
                            );
                        } else {
                            return Err(DbErr::RecordNotFound(format!(
                                "Market with base currency {base_currency} and quote currency {quote_currency} does not exist."
                            )))
                        }
                    },
                    (Some(base_currency), None) => {
                        if let Some(market) = markets::Entity::find()
                            .filter(markets::Column::BaseCurrency.eq(base_currency.to_uppercase()))
                            .one(db)
                            .await?
                        {
                            conditions = conditions.add(
                                orders::Column::MarketId.in_subquery(
                                    SeaQuery::select()
                                        .column(markets::Column::Id)
                                        .from(markets::Entity)
                                        .and_where(markets::Column::BaseCurrency.eq(market.base_currency))
                                        .to_owned(),
                                ),
                            );
                        } else {
                            return Err(DbErr::RecordNotFound(format!(
                                "Market with base currency {base_currency} does not exist."
                            )))
                        }
                    },
                    (None, Some(quote_currency)) => {
                        if let Some(market) = markets::Entity::find()
                            .filter(markets::Column::QuoteCurrency.eq(quote_currency.to_uppercase()))
                            .one(db)
                            .await?
                        {
                            conditions = conditions.add(
                                orders::Column::MarketId.in_subquery(
                                    SeaQuery::select()
                                        .column(markets::Column::Id)
                                        .from(markets::Entity)
                                        .and_where(markets::Column::QuoteCurrency.eq(market.quote_currency))
                                        .to_owned(),
                                ),
                            );
                        } else {
                            return Err(DbErr::RecordNotFound(format!(
                                "Market with quote currency {quote_currency} does not exist."
                            )))
                        }
                    },
                    (None, None) => {},
                }
            }

            let mut query = positions::Entity::find().filter(conditions);
            if let Some(side) = side {
                query = query.filter(positions::Column::Side.eq(side));
            }

            query
                .inner_join(sub_accounts::Entity)
                .column_as(sub_accounts::Column::Name, "sub_account")
                .inner_join(markets::Entity)
                .column(markets::Column::BaseCurrency)
                .column(markets::Column::QuoteCurrency)
                .column(markets::Column::PriceIncrement)
                .column(markets::Column::SizeIncrement)
                .into_model::<positions::Response>()
                .paginate(db, min(page_size.unwrap_or(1), 1000))
                .fetch_page(page.unwrap_or(1) - 1)
                .await
        } else {
            Err(DbErr::RecordNotFound(format!(
                "Client with id {client_id} does not exist."
            )))
        }
    }
    // ----------------------------------------------------------------------
}
