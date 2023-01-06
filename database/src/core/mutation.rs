use crate::entities::{clients, fills, markets, orders, positions, sub_accounts};
use crate::{Fill, OrderSide, OrderStatus, OrderType, SubAccountStatus};
use chrono::{Utc};
use sea_orm::prelude::*;
use sea_orm::*;
use sea_orm_migration::sea_query::Query as SeaQuery;

// ----------------------------------------------------------------------

pub struct Mutation;

impl Mutation {
    // Clients
    pub async fn create_client(db: &DbConn, email: String) -> Result<clients::Model, DbErr> {
        if let Some(_) = clients::Entity::find()
            .filter(clients::Column::Email.eq(email.clone()))
            .one(db)
            .await?
        {
            Err(DbErr::Custom(format!(
                "Client with email {email} already exists."
            )))
        } else {
            clients::ActiveModel {
                email: Set(email.to_owned()),
                created_at: Set(Utc::now().naive_utc()),
                ..Default::default()
            }
            .insert(db)
            .await
        }
    }

    pub async fn update_client(db: &DbConn, id: i32, new_email: String) -> Result<(), DbErr> {
        let other_client = clients::Entity::find()
            .filter(clients::Column::Email.eq(new_email.clone()))
            .one(db)
            .await?;
        let client = clients::Entity::find_by_id(id).one(db).await?;
        match (client, other_client) {
            (Some(client), None) => {
                let mut client: clients::ActiveModel = client.into();
                client.email = Set(new_email);
                let _ = client.update(db).await;
                Ok(())
            }
            (None, _) => Err(DbErr::RecordNotFound(format!(
                "Client with id {id} does not exist."
            ))),
            (_, Some(_)) => Err(DbErr::Custom(format!(
                "Client with email {new_email} already exists."
            ))),
        }
    }
    // ----------------------------------------------------------------------

    // Markets
    pub async fn create_market(
        db: &DbConn,
        base_currency: String,
        quote_currency: String,
        price_increment: f32,
        size_increment: f32,
    ) -> Result<markets::Model, DbErr> {
        if let Some(_) = markets::Entity::find()
            .filter(markets::Column::BaseCurrency.eq(base_currency.to_uppercase()))
            .filter(markets::Column::QuoteCurrency.eq(quote_currency.to_uppercase()))
            .one(db)
            .await?
        {
            Err(DbErr::Custom(format!(
                "Market with base currency {base_currency} and quote currency {quote_currency} already exists."
            )))
        } else {
            markets::ActiveModel {
                base_currency: Set(base_currency.to_owned()),
                quote_currency: Set(quote_currency.to_owned()),
                price_increment: Set(price_increment),
                size_increment: Set(size_increment),
                created_at: Set(Utc::now().naive_utc()),
                ..Default::default()
            }
            .insert(db)
            .await
        }
    }

    pub async fn update_market(
        db: &DbConn,
        market_id: i32,
        base_currency: Option<String>,
        quote_currency: Option<String>,
        price_increment: Option<f32>,
        size_increment: Option<f32>,
    ) -> Result<(), DbErr> {
        if let Some(market) = markets::Entity::find_by_id(market_id).one(db).await? {
            if let Some(other_market) = markets::Entity::find()
                .filter(markets::Column::BaseCurrency.eq(base_currency.clone())) // If base_currency = None, will return None
                .filter(markets::Column::QuoteCurrency.eq(quote_currency.clone())) // If quote_currency = None, will return None
                .one(db)
                .await?
            {
                Err(DbErr::RecordNotFound(format!(
                    "Market with base currency {} and quote currency {} already exists.",
                    other_market.base_currency, other_market.quote_currency
                )))
            } else {
                let mut market: markets::ActiveModel = market.into();
                if let Some(base_currency) = base_currency {
                    market.base_currency = Set(base_currency);
                }
                if let Some(quote_currency) = quote_currency {
                    market.quote_currency = Set(quote_currency);
                }
                if let Some(price_increment) = price_increment {
                    market.price_increment = Set(price_increment);
                }
                if let Some(size_increment) = size_increment {
                    market.size_increment = Set(size_increment);
                }
                let _ = market.update(db).await;
                Ok(())
            }
        } else {
            Err(DbErr::RecordNotFound(format!(
                "Market with id {market_id} does not exist."
            )))
        }
    }

    // ----------------------------------------------------------------------

    // SubAccounts
    pub async fn create_sub_account(
        db: &DbConn,
        client_id: i32,
        name: String,
    ) -> Result<sub_accounts::Model, DbErr> {
        if let Some(client) = clients::Entity::find_by_id(client_id.clone())
            .one(db)
            .await?
        {
            if let None = client.find_related(sub_accounts::Entity)
                .filter(sub_accounts::Column::Name.eq(name.clone()))
                .filter(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                .one(db)
                .await?
            {
                sub_accounts::ActiveModel {
                    name: Set(name),
                    created_at: Set(Utc::now().naive_utc()),
                    client_id: Set(client_id),
                    status: Set(SubAccountStatus::Active),
                    ..Default::default()
                }
                    .insert(db)
                    .await
            } else {
                Err(DbErr::Custom(format!(
                    "Sub-account with name {name} already exists."
                )))
            }
        } else {
            Err(DbErr::RecordNotFound(format!(
                "Client with id {client_id} does not exist."
            )))
        }
    }

    pub async fn update_sub_account(
        db: &DbConn,
        client_id: i32,
        sub_account_id: i32,
        name: Option<String>,
        status: Option<SubAccountStatus>,
    ) -> Result<(), DbErr> {
        if let Some(client) = clients::Entity::find_by_id(client_id).one(db).await? {
            if let Some(sub_account) = client.find_related(sub_accounts::Entity)
                .filter(sub_accounts::Column::Id.eq(sub_account_id))
                .filter(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                .one(db)
                .await?
            {
                let mut sub_account: sub_accounts::ActiveModel = sub_account.into_active_model();
                if let Some(name) = name {
                    if let None = client.find_related(sub_accounts::Entity)
                        .filter(sub_accounts::Column::Name.eq(name.clone()))
                        .filter(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                        .one(db)
                        .await?
                    {
                        sub_account.name = Set(name.to_owned())
                    } else {
                        return Err(DbErr::Custom(format!(
                            "Sub-account with name {name} already exists."
                        )))
                    }
                }
                if let Some(status) = status {
                    sub_account.status = Set(status)
                }
                let _ = sub_account.update(db).await;
                Ok(())
            } else {
                Err(DbErr::RecordNotFound(format!(
                    "Sub-account with id {sub_account_id} does not exist."
                )))
            }
        } else {
            Err(DbErr::RecordNotFound(format!(
                "Client with id {client_id} does not exist."
            )))
        }
    }
    // ----------------------------------------------------------------------

    // Orders
    pub async fn update_order(
        db: &DbConn,
        order: orders::ActiveModel,
    ) -> Result<(), DbErr> {
        let _ = order.update(db).await?;
        Ok(())
    }

    pub async fn update_order_from_fill(
        db: &DbConn,
        fill: fills::Model,
    ) -> Result<(), DbErr> {
        if let Some(order) = fill.find_related(orders::Entity)
            .filter(orders::Column::Status.eq(OrderStatus::Open))
            .filter(orders::Column::SubAccountId.eq(fill.sub_account_id))
            .filter(orders::Column::MarketId.eq(fill.market_id))
            .one(db)
            .await?
        {
            let mut order = order.into_active_model();
            order.filled_size = Set(order.filled_size.unwrap() + fill.size);
            if order.filled_size == order.size {
                order.status = Set(OrderStatus::Closed);
                order.closed_at = Set(Some(Utc::now().naive_utc()));
            }
            Ok(())
        } else {
            Err(DbErr::RecordNotFound(format!(
                "Found no order matching fill {:?}", fill
            )))
        }
    }

    pub async fn create_order(
        db: &DbConn,
        client_id: i32,
        sub_account_id: i32,
        size: f32,
        side: OrderSide,
        r#type: OrderType,
        price: Option<f32>,
        client_order_id: Option<String>,
        market_id: Option<i32>,
        base_currency: Option<String>,
        quote_currency: Option<String>,
    ) -> Result<orders::Model, DbErr> {
        let sub_account_and_client: Option<(sub_accounts::Model, Option<clients::Model>)> =
            sub_accounts::Entity::find_by_id(sub_account_id)
                .filter(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                .find_also_related(clients::Entity)
                .one(db)
                .await?;
        let market = if let Some(market_id) = market_id {
             markets::Entity::find_by_id(market_id)
                .one(db)
                .await?
        } else {
            match (base_currency, quote_currency) {
                (Some(base_currency), Some(quote_currency)) => {
                    markets::Entity::find()
                        .filter(markets::Column::BaseCurrency.eq(base_currency.to_uppercase()))
                        .filter(markets::Column::QuoteCurrency.eq(quote_currency.to_uppercase()))
                        .one(db)
                        .await?
                },
                _ => return Err(DbErr::Custom(format!(
                    "Missing query arguments."
                ))),
            }
        };
        match (sub_account_and_client, market) {
            (Some((sub_account, Some(_))), Some(market)) => {
                let price: ActiveValue<Option<f32>> = if let Some(price) = price {
                    if price < market.price_increment || r#type == OrderType::Market || size < market.size_increment {
                        return Err(DbErr::Custom(format!(
                            "Invalid order parameters."
                        )))
                    }
                    Set(Some(price))
                } else {
                    if r#type == OrderType::Limit || size < market.size_increment {
                        return Err(DbErr::Custom(format!(
                            "Invalid order parameters."
                        )))
                    }
                    NotSet
                };
                orders::ActiveModel {
                    client_order_id: Set(client_order_id),
                    price,
                    size: Set(size),
                    filled_size: Set(0.0),
                    side: Set(side),
                    r#type: Set(r#type),
                    status: Set(OrderStatus::Open),
                    open_at: Set(Utc::now().naive_utc()),
                    closed_at: NotSet,
                    sub_account_id: Set(sub_account.id),
                    market_id: Set(market.id),
                    ..Default::default()
                }
                    .insert(db)
                    .await
            }
            (None, _) => Err(DbErr::RecordNotFound(format!(
                "Sub-account with id {sub_account_id} does not exist."
            ))),
            (Some((_, None)), _) => Err(DbErr::RecordNotFound(format!(
                "Client with id {client_id} does not exist."
            ))),
            (_, None) => Err(DbErr::RecordNotFound(format!(
                "Market does not exist."
            ))),
        }
    }
    // ----------------------------------------------------------------------

    // Fills
    pub async fn create_fill(
        db: &DbConn,
        fill: fills::Model,
    ) -> Result<Fill, DbErr> {
        let fill = fill.into_active_model().insert(db).await?;
        let sub_account = fill.find_related(sub_accounts::Entity)
            .select_only()
            .column(sub_accounts::Column::Name)
            .one(db)
            .await?
            .unwrap();
        let market = fill.find_related(markets::Entity).one(db).await?.unwrap();
        Ok(Fill {
            price: fill.price,
            size: fill.size,
            quote_size: fill.quote_size,
            side: fill.side,
            r#type: fill.r#type,
            created_at: fill.created_at,
            base_currency: market.base_currency,
            quote_currency: market.quote_currency,
            price_increment: market.price_increment,
            size_increment: market.size_increment,
            sub_account: sub_account.name,
            order_id: fill.order_id,
        })
    }
    // ----------------------------------------------------------------------

    // Positions
    pub async fn upsert_position_from_fill(
        db: &DbConn,
        fill: fills::Model,
    ) -> Result<(), DbErr> {
        if let Some(position) = positions::Entity::find()
            .filter(
                Condition::all().add(
                    positions::Column::SubAccountId.in_subquery(
                        SeaQuery::select()
                            .column(sub_accounts::Column::Id)
                            .from(sub_accounts::Entity)
                            .and_where(sub_accounts::Column::Id.eq(fill.sub_account_id.clone()))
                            .and_where(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                            .to_owned()
                    )
                ).add(
                    positions::Column::MarketId.in_subquery(
                        SeaQuery::select()
                            .column(markets::Column::Id)
                            .from(markets::Entity)
                            .and_where(markets::Column::Id.eq(fill.market_id.clone()))
                            .to_owned()
                    )
                )
            )
            .one(db).await?
        {
            let mut position = position.into_active_model();
            position.size = Set(position.size.unwrap() + fill.size.clone());
            position.avg_entry_price = Set(
                (
                    position.avg_entry_price.unwrap() * position.size.clone().unwrap()
                        + fill.price.clone() * fill.size
                ) / position.size.clone().unwrap()
            );
            let _ = position.update(db).await;
        } else {
            positions::ActiveModel {
                avg_entry_price: Set(fill.price),
                size: Set(fill.size),
                side: Set(fill.side),
                sub_account_id: Set(fill.sub_account_id),
                market_id: Set(fill.market_id),
                ..Default::default()
            }
                .insert(db)
                .await?;
        }
        Ok(())
    }
    // ----------------------------------------------------------------------
}
