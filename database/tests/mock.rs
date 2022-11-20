mod prepare;
use database::{clients, markets, sub_accounts, Query, Order, sea_orm_active_enums::{OrderSide, OrderType, OrderStatus}};
use prepare::prepare_mock_db;

// ----------------------------------------------------------------------

#[async_std::test]
async fn main() {
    let db = &prepare_mock_db();

    // Clients
    {
        assert_eq!(
            Query::find_client_by_id(db, 1).await.unwrap().unwrap(),
            clients::Model {
                id: 1,
                email: "ivanjericevich96@gmail.com".to_owned(),
                created_at: "2022-01-01T00:00:00".parse().unwrap(),
            }
        );
    }

    {
        assert_eq!(
            Query::find_client_by_email(db, "ivanjericevich96@gmail.com".to_owned()).await.unwrap().unwrap(),
            clients::Model {
                id: 1,
                email: "ivanjericevich96@gmail.com".to_owned(),
                created_at: "2022-01-01T00:00:00".parse().unwrap(),
            }
        );
    }
    // ----------------------------------------------------------------------

    // Markets
    {
        assert_eq!(
            Query::find_market_by_id(db, 1).await.unwrap().unwrap(),
            markets::Model {
                id: 1,
                base_currency: "BTC".to_owned(),
                quote_currency: "USD".to_owned(),
                price_increment: 0.01,
                size_increment: 0.01,
                created_at: "2022-01-01T00:00:00".parse().unwrap(),
            }
        );
    }

    {
        assert_eq!(
            Query::find_market_by_ticker(db, "BTC".to_owned(), "USD".to_owned()).await.unwrap().unwrap(),
            markets::Model {
                id: 1,
                base_currency: "BTC".to_owned(),
                quote_currency: "USD".to_owned(),
                price_increment: 0.01,
                size_increment: 0.01,
                created_at: "2022-01-01T00:00:00".parse().unwrap(),
            }
        );
    }
    // ----------------------------------------------------------------------

    // SubAccounts
    {
        assert_eq!(
            Query::find_sub_account_by_id(db, 1).await.unwrap().unwrap(),
            sub_accounts::Model {
                id: 1,
                name: "Test".to_owned(),
                created_at: "2022-01-01T00:00:00".parse().unwrap(),
                client_id: 1,
            }
        );
    }

    {
        assert_eq!(
            Query::find_sub_account_by_client_id(db, 1).await.unwrap(),
            vec![(
                clients::Model {
                    id: 1,
                    email: "ivanjericevich96@gmail.com".to_owned(),
                    created_at: "2022-01-01T00:00:00".parse().unwrap(),
                },
                vec![
                    sub_accounts::Model {
                        id: 1,
                        name: "Test".to_owned(),
                        created_at: "2022-01-01T00:00:00".parse().unwrap(),
                        client_id: 1,
                    }
                ]
            )]

        );
    }
    // ----------------------------------------------------------------------

    // Orders
    // ----------------------------------------------------------------------

    // TODO: Create tests for failures (e.g. None returns)
}