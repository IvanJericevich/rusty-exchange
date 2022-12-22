use crate::entities::{clients, markets, sub_accounts};
use crate::SubAccountStatus;
use chrono::Utc;
use sea_orm::prelude::*;
use sea_orm::ActiveValue::Set;

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
                "Market with symbol {base_currency}/{quote_currency} already exists."
            )))
        } else {
            markets::ActiveModel {
                base_currency: Set(base_currency.to_owned()),
                quote_currency: Set(base_currency.to_owned()),
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
            let mut market: markets::ActiveModel = market.into();
            if base_currency.is_some() {
                market.base_currency = Set(base_currency.unwrap())
            }
            if quote_currency.is_some() {
                market.quote_currency = Set(quote_currency.unwrap())
            }
            if price_increment.is_some() {
                market.price_increment = Set(price_increment.unwrap())
            }
            if size_increment.is_some() {
                market.size_increment = Set(size_increment.unwrap())
            }
            let _ = market.update(db).await;
            Ok(())
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
        let client = clients::Entity::find_by_id(client_id.clone())
            .one(db)
            .await?;
        let sub_account = sub_accounts::Entity::find()
            .filter(sub_accounts::Column::Name.eq(name.clone()))
            .one(db)
            .await?;
        match (client, sub_account) {
            (Some(_), None) => {
                sub_accounts::ActiveModel {
                    name: Set(name),
                    created_at: Set(Utc::now().naive_utc()),
                    client_id: Set(client_id),
                    status: Set(SubAccountStatus::Active),
                    ..Default::default()
                }
                .insert(db)
                .await
            }
            (None, _) => Err(DbErr::RecordNotFound(format!(
                "Client with id {client_id} does not exist."
            ))),
            (_, Some(_)) => Err(DbErr::Custom(format!(
                "Sub-account with name {name} already exists."
            ))),
        }
    }

    pub async fn update_sub_account(
        db: &DbConn,
        client_id: i32,
        sub_account_id: i32,
        name: Option<String>,
        status: Option<SubAccountStatus>,
    ) -> Result<(), DbErr> {
        let client = clients::Entity::find_by_id(client_id).one(db).await?;
        let sub_account: Option<sub_accounts::Model> =
            sub_accounts::Entity::find_by_id(sub_account_id)
                .filter(sub_accounts::Column::Status.eq(SubAccountStatus::Active))
                .one(db)
                .await?;
        match (client, sub_account) {
            (Some(_), Some(sub_account)) => {
                let mut sub_account: sub_accounts::ActiveModel = sub_account.into();
                if name.is_some() {
                    sub_account.name = Set(name.unwrap().to_owned())
                }
                if status.is_some() {
                    sub_account.status = Set(status.unwrap())
                }
                let _ = sub_account.update(db).await;
                Ok(())
            }
            (None, _) => Err(DbErr::RecordNotFound(format!(
                "Client with id {client_id} does not exist."
            ))),
            (_, None) => Err(DbErr::RecordNotFound(format!(
                "Sub-account with id {sub_account_id} does not exist."
            ))),
        }
    }
    // ----------------------------------------------------------------------

    // Orders
    // pub async fn create_order(
    //     db: &DbConn,
    //     order: orders::ActiveModel,
    // ) -> Result<orders::Model, DbErr> {
    //     order.insert(db).await
    // }

    // pub async fn create_orders(
    //     db: &DbConn,
    //     orders: Vec<orders::ActiveModel>,
    // ) -> Result<orders::Model, DbErr> {
    //     let res: InsertResult = orders::Model::insert_many(orders).exec(db).await?;
    //
    // }

    // pub async fn update_order_by_id(
    //     db: &DbConn,
    //     order_id: i32,
    //     price: Option<f32>,
    //     size: Option<f32>,
    //     filled_size: Option<f32>,
    //     closed_at: Option<DateTime>,
    //     status: Option<OrderStatus>,
    // ) -> Result<(), DbErr> {
    //     let order: Option<orders::Model> = orders::Entity::find_by_id(order_id).one(db).await?;
    //
    //     match order {
    //         Some(order) => {
    //             let mut order: orders::ActiveModel = order.into();
    //             if price.is_some() && price > Some(0.0) {
    //                 order.price = Set(price.unwrap());
    //             }
    //             if size.is_some() && size > Some(0.0) {
    //                 order.size = Set(size.unwrap());
    //             }
    //             if filled_size.is_some() && filled_size > Some(0.0) {
    //                 order.filled_size = Set(filled_size);
    //             }
    //             order.closed_at = Set(closed_at);
    //             if status.is_some() {
    //                 order.status = Set(status.unwrap());
    //             }
    //             Ok(())
    //         }
    //         None => Err(DbErr::RecordNotFound(format!(
    //             "Order with id {order_id} does not exist."
    //         ))),
    //     }
    // }

    // pub async fn update_order_by_client_order_id(
    //     // TODO: What about updating open orders or market/limit orders
    //     db: &DbConn,
    //     client_order_id: i32,
    //     price: Option<f32>,
    //     size: Option<f32>,
    //     filled_size: Option<f32>,
    //     closed_at: Option<DateTime>,
    //     status: Option<OrderStatus>,
    // ) -> Result<(), DbErr> {
    //     let order: Option<orders::Model> = orders::Entity::find()
    //         .filter(orders::Column::ClientOrderId.eq(client_order_id))
    //         .one(db)
    //         .await?;
    //
    //     match order {
    //         Some(order) => {
    //             let mut order: orders::ActiveModel = order.into();
    //             if price.is_some() && price > Some(0.0) {
    //                 // TODO: use is_some_and instead
    //                 order.price = Set(price.unwrap());
    //             }
    //             if size.is_some() && size > Some(0.0) {
    //                 order.size = Set(size.unwrap());
    //             }
    //             if filled_size.is_some() && filled_size > Some(0.0) {
    //                 order.filled_size = Set(filled_size);
    //             }
    //             order.closed_at = Set(closed_at);
    //             if status.is_some() {
    //                 order.status = Set(status.unwrap());
    //             }
    //             Ok(())
    //         }
    //         None => Err(DbErr::RecordNotFound(format!(
    //             "Order with client order id {client_order_id} does not exist."
    //         ))),
    //     }
    // }

    // pub async fn update_orders(
    //     db: &DbConn,
    //     orders: Vec<UpdateOrderRequest>,
    // ) -> Result<orders::Model, DbErr> {
    //     todo!()
    // }

    // pub async fn delete_order_by_order_id(db: &DbConn, order_id: i32) -> Result<(), DbErr> {
    //     todo!()
    // }
    //
    // pub async fn delete_order_by_client_order_id(
    //     db: &DbConn,
    //     client_order_id: i32,
    // ) -> Result<(), DbErr> {
    //     todo!()
    // }
    //
    // pub async fn delete_orders(db: &DbConn) -> Result<(), DbErr> {
    //     todo!()
    // }
    // ----------------------------------------------------------------------

    // Positions
    // pub async fn create_position(
    //     db: &DbConn,
    //     client_id: i32,
    //     sub_account_id: i32,
    // ) -> Result<orders::Model, DbErr> {
    //     todo!()
    // }
    //
    // pub async fn create_positions(
    //     db: &DbConn,
    //     client_id: i32,
    //     sub_account_id: i32,
    // ) -> Result<Vec<orders::Model>, DbErr> {
    //     todo!()
    // }
    //
    // pub async fn update_position(
    //     db: &DbConn,
    //     client_id: i32,
    //     sub_account_id: i32,
    // ) -> Result<(), DbErr> {
    //     todo!()
    // }
    //
    // pub async fn update_positions(
    //     db: &DbConn,
    //     client_id: i32,
    //     sub_account_id: i32,
    // ) -> Result<(), DbErr> {
    //     todo!()
    // }
    //
    // pub async fn delete_position(
    //     db: &DbConn,
    //     client_id: i32,
    //     sub_account_id: i32,
    // ) -> Result<(), DbErr> {
    //     todo!()
    // }
    //
    // pub async fn delete_positions(
    //     db: &DbConn,
    //     client_id: i32,
    //     sub_account_id: i32,
    // ) -> Result<(), DbErr> {
    //     todo!()
    // }
    // ----------------------------------------------------------------------
}
