use database::{Client, DbErr, Market, Query, SubAccount, SubAccountStatus};
use sea_orm::{DatabaseBackend, MockDatabase};

// ----------------------------------------------------------------------

#[async_std::test]
async fn clients() {
    let db = &MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![
            vec![
                Client {
                    id: 1,
                    email: "ivanjericevich96@gmail.com".to_owned(),
                    created_at: "2022-01-01T00:00:00".parse().unwrap(),
                },
                Client {
                    id: 2,
                    email: "ivanjericevich@gmail.com".to_owned(),
                    created_at: "2022-01-01T00:00:00".parse().unwrap(),
                },
            ],
            vec![],
            vec![
                Client {
                    id: 1,
                    email: "ivanjericevich96@gmail.com".to_owned(),
                    created_at: "2022-01-01T00:00:00".parse().unwrap(),
                },
                Client {
                    id: 2,
                    email: "ivanjericevich@gmail.com".to_owned(),
                    created_at: "2022-01-01T00:00:00".parse().unwrap(),
                },
            ],
            vec![],
        ])
        .into_connection();

    // Find one by id
    assert_eq!(
        Query::find_client_by_id(db, 1).await.unwrap(),
        Client {
            id: 1,
            email: "ivanjericevich96@gmail.com".to_owned(),
            created_at: "2022-01-01T00:00:00".parse().unwrap(),
        }
    );
    // Find None by id
    assert_eq!(
        Query::find_client_by_id(db, 3).await.unwrap_err(),
        DbErr::RecordNotFound("Client with id 3 does not exist.".to_owned())
    );
    // Find one by email
    assert_eq!(
        Query::find_client_by_email(db, "ivanjericevich96@gmail.com".to_owned())
            .await
            .unwrap(),
        Client {
            id: 1,
            email: "ivanjericevich96@gmail.com".to_owned(),
            created_at: "2022-01-01T00:00:00".parse().unwrap(),
        }
    );
    // Find None by email
    assert_eq!(
        Query::find_client_by_email(db, "ivan@gmail.com".to_owned())
            .await
            .unwrap_err(),
        DbErr::RecordNotFound("Client with email ivan@gmail.com does not exist.".to_owned())
    );
}

// ----------------------------------------------------------------------

#[async_std::test]
async fn markets() {
    let db = &MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![
            vec![
                Market {
                    id: 1,
                    base_currency: "BTC".to_owned(),
                    quote_currency: "USD".to_owned(),
                    price_increment: 0.01,
                    size_increment: 0.01,
                    created_at: "2022-01-01T00:00:00".parse().unwrap(),
                },
                Market {
                    id: 2,
                    base_currency: "ETH".to_owned(),
                    quote_currency: "USD".to_owned(),
                    price_increment: 0.01,
                    size_increment: 0.01,
                    created_at: "2022-01-01T00:00:00".parse().unwrap(),
                },
            ],
            vec![],
            vec![
                Market {
                    id: 1,
                    base_currency: "BTC".to_owned(),
                    quote_currency: "USD".to_owned(),
                    price_increment: 0.01,
                    size_increment: 0.01,
                    created_at: "2022-01-01T00:00:00".parse().unwrap(),
                },
                Market {
                    id: 2,
                    base_currency: "ETH".to_owned(),
                    quote_currency: "USD".to_owned(),
                    price_increment: 0.01,
                    size_increment: 0.01,
                    created_at: "2022-01-01T00:00:00".parse().unwrap(),
                },
            ],
            vec![],
        ])
        .into_connection();

    // Find one by id
    assert_eq!(
        Query::find_market_by_id(db, 1).await.unwrap(),
        Market {
            id: 1,
            base_currency: "BTC".to_owned(),
            quote_currency: "USD".to_owned(),
            price_increment: 0.01,
            size_increment: 0.01,
            created_at: "2022-01-01T00:00:00".parse().unwrap(),
        }
    );
    // Find None by id
    assert_eq!(
        Query::find_market_by_id(db, 3).await.unwrap_err(),
        DbErr::RecordNotFound("Market with id 3 does not exist.".to_owned())
    );
    // Find one by ticker
    assert_eq!(
        Query::find_market_by_ticker(db, "BTC".to_owned(), "USD".to_owned())
            .await
            .unwrap(),
        Market {
            id: 1,
            base_currency: "BTC".to_owned(),
            quote_currency: "USD".to_owned(),
            price_increment: 0.01,
            size_increment: 0.01,
            created_at: "2022-01-01T00:00:00".parse().unwrap(),
        }
    );
    // Find None by ticker
    assert_eq!(
        Query::find_market_by_ticker(db, "BTC".to_owned(), "USD".to_owned())
            .await
            .unwrap_err(),
        DbErr::RecordNotFound("Market with symbol BTC/USD does not exist.".to_owned())
    );
}

// ----------------------------------------------------------------------

#[async_std::test]
async fn sub_accounts() {
    let db = &MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![SubAccount {
            id: 1,
            name: "Test".to_owned(),
            created_at: "2022-01-01T00:00:00".parse().unwrap(),
            client_id: 1,
            status: SubAccountStatus::Active,
        }]])
        .append_query_results(vec![vec![Client {
            id: 1,
            email: "ivanjericevich96@gmail.com".to_owned(),
            created_at: "2022-01-01T00:00:00".parse().unwrap(),
        }]])
        .append_query_results(vec![vec![SubAccount {
            id: 1,
            name: "Test".to_owned(),
            created_at: "2022-01-01T00:00:00".parse().unwrap(),
            client_id: 1,
            status: SubAccountStatus::Active,
        }]])
        .into_connection();

    assert_eq!(
        Query::find_sub_account_by_id(db, 1).await.unwrap(),
        SubAccount {
            id: 1,
            name: "Test".to_owned(),
            created_at: "2022-01-01T00:00:00".parse().unwrap(),
            client_id: 1,
            status: SubAccountStatus::Active
        }
    );
    assert_eq!(
        Query::find_sub_accounts_by_client_id(db, 1).await.unwrap(),
        vec![SubAccount {
            id: 1,
            name: "Test".to_owned(),
            created_at: "2022-01-01T00:00:00".parse().unwrap(),
            client_id: 1,
            status: SubAccountStatus::Active
        }]
    );
}

// ----------------------------------------------------------------------
