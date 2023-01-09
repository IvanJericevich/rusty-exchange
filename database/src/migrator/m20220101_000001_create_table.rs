use sea_orm::sea_query::extension::postgres::Type;
use sea_orm_migration::prelude::*;

// ----------------------------------------------------------------------

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_tables"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(SubAccountStatus::Table)
                    .values([SubAccountStatus::Active, SubAccountStatus::Inactive])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(OrderSide::Table)
                    .values([
                        OrderSide::Buy,
                        OrderSide::Long,
                        OrderSide::Bid,
                        OrderSide::Sell,
                        OrderSide::Short,
                        OrderSide::Ask
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(OrderType::Table)
                    .values([OrderType::Market, OrderType::Limit])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(OrderStatus::Table)
                    .values([OrderStatus::Open, OrderStatus::Closed])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Clients::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Clients::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Clients::Email).string().unique_key().not_null())
                    .col(ColumnDef::new(Clients::CreatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Markets::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Markets::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Markets::BaseCurrency).string().not_null())
                    .col(ColumnDef::new(Markets::QuoteCurrency).string().not_null())
                    .col(ColumnDef::new(Markets::PriceIncrement).float().not_null())
                    .col(ColumnDef::new(Markets::SizeIncrement).float().not_null())
                    .col(ColumnDef::new(Markets::CreatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(SubAccounts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SubAccounts::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SubAccounts::Name).string().not_null())
                    .col(
                        ColumnDef::new(SubAccounts::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(ColumnDef::new(SubAccounts::ClientId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("client_id")
                            .from(SubAccounts::Table, SubAccounts::ClientId)
                            .to(Clients::Table, Clients::Id),
                    )
                    .col(
                        ColumnDef::new(SubAccounts::Status)
                            .enumeration(
                                SubAccountStatus::Table,
                                [SubAccountStatus::Active, SubAccountStatus::Inactive],
                            )
                            .not_null()
                            .default("active"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Orders::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Orders::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Orders::ClientOrderId).string())
                    .col(ColumnDef::new(Orders::Price).float())
                    .col(ColumnDef::new(Orders::Size).float().not_null())
                    .col(ColumnDef::new(Orders::FilledSize).float().not_null())
                    .col(
                        ColumnDef::new(Orders::Side)
                            .enumeration(
                                OrderSide::Table,
                                [
                                    OrderSide::Buy,
                                    OrderSide::Long,
                                    OrderSide::Bid,
                                    OrderSide::Sell,
                                    OrderSide::Short,
                                    OrderSide::Ask
                                ]
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Orders::Type)
                            .enumeration(OrderType::Table, [OrderType::Market, OrderType::Limit])
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Orders::Status)
                            .enumeration(
                                OrderStatus::Table,
                                [OrderStatus::Open, OrderStatus::Closed],
                            )
                            .not_null(),
                    )
                    .col(ColumnDef::new(Orders::OpenAt).timestamp().not_null())
                    .col(ColumnDef::new(Orders::ClosedAt).timestamp())
                    .col(ColumnDef::new(Orders::SubAccountId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("sub_account_id")
                            .from(Orders::Table, Orders::SubAccountId)
                            .to(SubAccounts::Table, SubAccounts::Id),
                    )
                    .col(ColumnDef::new(Orders::MarketId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("market_id")
                            .from(Orders::Table, Orders::MarketId)
                            .to(Markets::Table, Markets::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Fills::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Fills::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Fills::Price).float().not_null())
                    .col(ColumnDef::new(Fills::Size).float().not_null())
                    .col(ColumnDef::new(Fills::QuoteSize).float().not_null())
                    .col(
                        ColumnDef::new(Fills::Side)
                            .enumeration(
                                OrderSide::Table,
                                [
                                    OrderSide::Buy,
                                    OrderSide::Long,
                                    OrderSide::Bid,
                                    OrderSide::Sell,
                                    OrderSide::Short,
                                    OrderSide::Ask
                                ]
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Fills::Type)
                            .enumeration(OrderType::Table, [OrderType::Market, OrderType::Limit])
                            .not_null(),
                    )
                    .col(ColumnDef::new(Fills::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Fills::SubAccountId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("sub_account_id")
                            .from(Fills::Table, Fills::SubAccountId)
                            .to(SubAccounts::Table, SubAccounts::Id),
                    )
                    .col(ColumnDef::new(Fills::MarketId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("market_id")
                            .from(Fills::Table, Fills::MarketId)
                            .to(Markets::Table, Markets::Id),
                    )
                    .col(ColumnDef::new(Fills::OrderId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("order_id")
                            .from(Fills::Table, Fills::OrderId)
                            .to(Orders::Table, Orders::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Positions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Positions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Positions::AvgEntryPrice).float().not_null())
                    .col(ColumnDef::new(Positions::Size).float().not_null())
                    .col(
                        ColumnDef::new(Positions::Side)
                            .enumeration(
                                OrderSide::Table,
                                [
                                    OrderSide::Buy,
                                    OrderSide::Long,
                                    OrderSide::Bid,
                                    OrderSide::Sell,
                                    OrderSide::Short,
                                    OrderSide::Ask
                                ]
                            )
                            .not_null(),
                    )
                    .col(ColumnDef::new(Orders::SubAccountId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("sub_account_id")
                            .from(Positions::Table, Positions::SubAccountId)
                            .to(SubAccounts::Table, SubAccounts::Id),
                    )
                    .col(ColumnDef::new(Orders::MarketId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("market_id")
                            .from(Positions::Table, Positions::MarketId)
                            .to(Markets::Table, Markets::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(SubAccounts::Table)
                    .name("client_id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Orders::Table)
                    .name("sub_account_id")
                    .name("market_id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Positions::Table)
                    .name("sub_account_id")
                    .name("market_id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(Clients::Table)
                    .table(Markets::Table)
                    .table(SubAccounts::Table)
                    .table(Fills::Table)
                    .table(Orders::Table)
                    .table(Positions::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(Type::drop().name(SubAccountStatus::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(OrderSide::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(OrderType::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(OrderStatus::Table).to_owned())
            .await?;

        Ok(())
    }
}

// ----------------------------------------------------------------------

#[derive(Iden)]
enum Clients {
    Table,
    Id, // Primary key
    Email,
    CreatedAt,
}

#[derive(Iden)]
enum Markets {
    Table,
    Id, // Primary key
    BaseCurrency,
    QuoteCurrency,
    PriceIncrement,
    SizeIncrement,
    CreatedAt,
}

#[derive(Iden)]
pub enum SubAccountStatus {
    Table,
    #[iden = "active"]
    Active,
    #[iden = "inactive"]
    Inactive,
}

#[derive(Iden)]
enum SubAccounts {
    Table,
    Id, // Primary key
    Name,
    CreatedAt,
    ClientId, // Foreign key
    Status,
}

#[derive(Iden)]
pub enum OrderSide {
    Table,
    #[iden = "buy"]
    Buy,
    #[iden = "long"]
    Long,
    #[iden = "bid"]
    Bid,
    #[iden = "sell"]
    Sell,
    #[iden = "short"]
    Short,
    #[iden = "ask"]
    Ask,
}

#[derive(Iden)]
pub enum OrderType {
    Table,
    #[iden = "market"]
    Market,
    #[iden = "limit"]
    Limit,
}

#[derive(Iden)]
pub enum OrderStatus {
    Table,
    #[iden = "open"]
    Open,
    #[iden = "closed"]
    Closed,
}

#[derive(Iden)]
enum Orders {
    Table,
    Id, // Primary key
    ClientOrderId,
    Price,
    Size,
    FilledSize,
    Side,
    Type,
    Status,
    OpenAt,
    ClosedAt,
    SubAccountId, // Foreign key
    MarketId, // Foreign key
}

#[derive(Iden)]
enum Fills {
    Table,
    Id, // Primary key
    Price,
    Size,
    QuoteSize,
    Side,
    Type,
    CreatedAt,
    SubAccountId, // Foreign key
    MarketId, // Foreign key
    OrderId // Foreign key
}

#[derive(Iden)]
enum Positions {
    Table,
    Id, // Primary key
    AvgEntryPrice,
    Size,
    Side,
    SubAccountId, // Foreign key
    MarketId, // Foreign key
}
