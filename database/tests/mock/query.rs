use database::{clients, markets, sub_accounts, DbErr, Query, SubAccountStatus};
use sea_orm::{DatabaseBackend, MockDatabase};

// ----------------------------------------------------------------------

#[async_std::test]
pub async fn clients() {
    let db = &MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![
            vec![
                clients::Model {
                    id: 1,
                    email: "ivanjericevich96@gmail.com".to_owned(),
                    created_at: "2022-01-01T00:00:00".parse().unwrap(),
                },
                clients::Model {
                    id: 2,
                    email: "ivanjericevich@gmail.com".to_owned(),
                    created_at: "2022-01-01T00:00:00".parse().unwrap(),
                },
            ],
            vec![],
            vec![
                clients::Model {
                    id: 1,
                    email: "ivanjericevich96@gmail.com".to_owned(),
                    created_at: "2022-01-01T00:00:00".parse().unwrap(),
                },
                clients::Model {
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
        clients::Model {
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
        clients::Model {
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
pub async fn markets() {
    let db = &MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![
            vec![
                markets::Model {
                    id: 1,
                    base_currency: "BTC".to_owned(),
                    quote_currency: "USD".to_owned(),
                    price_increment: 0.01,
                    size_increment: 0.01,
                    created_at: "2022-01-01T00:00:00".parse().unwrap(),
                },
                markets::Model {
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
                markets::Model {
                    id: 1,
                    base_currency: "BTC".to_owned(),
                    quote_currency: "USD".to_owned(),
                    price_increment: 0.01,
                    size_increment: 0.01,
                    created_at: "2022-01-01T00:00:00".parse().unwrap(),
                },
                markets::Model {
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
        markets::Model {
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
        markets::Model {
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
pub async fn sub_accounts() {
    let empty_sub_account_vector: Vec<sub_accounts::Model> = vec![];
    let db = &MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![
            vec![sub_accounts::Model {
                id: 1,
                name: "Test".to_owned(),
                created_at: "2022-01-01T00:00:00".parse().unwrap(),
                client_id: 1,
                status: SubAccountStatus::Active,
            }],
            vec![],
        ])
        .append_query_results(vec![vec![clients::Model {
            id: 1,
            email: "ivanjericevich96@gmail.com".to_owned(),
            created_at: "2022-01-01T00:00:00".parse().unwrap(),
        }]])
        .append_query_results(vec![vec![sub_accounts::Model {
            id: 1,
            name: "Test".to_owned(),
            created_at: "2022-01-01T00:00:00".parse().unwrap(),
            client_id: 1,
            status: SubAccountStatus::Active,
        }]])
        .append_query_results(vec![vec![clients::Model {
            id: 1,
            email: "ivanjericevich96@gmail.com".to_owned(),
            created_at: "2022-01-01T00:00:00".parse().unwrap(),
        }]])
        .append_query_results(vec![empty_sub_account_vector])
        .into_connection();
    // Find one by id
    assert_eq!(
        Query::find_sub_account_by_id(db, 1).await.unwrap(),
        sub_accounts::Model {
            id: 1,
            name: "Test".to_owned(),
            created_at: "2022-01-01T00:00:00".parse().unwrap(),
            client_id: 1,
            status: SubAccountStatus::Active
        }
    );
    // Find None by id
    assert_eq!(
        Query::find_sub_account_by_id(db, 1).await.unwrap_err(),
        DbErr::RecordNotFound("Sub-account with id 1 does not exist.".to_owned())
    );
    // Find some by client_id
    assert_eq!(
        Query::find_sub_accounts_by_client_id(db, 1).await.unwrap(),
        vec![sub_accounts::Model {
            id: 1,
            name: "Test".to_owned(),
            created_at: "2022-01-01T00:00:00".parse().unwrap(),
            client_id: 1,
            status: SubAccountStatus::Active
        }]
    );
    // Find None by client_id
    assert_eq!(
        Query::find_sub_accounts_by_client_id(db, 1).await.unwrap(),
        vec![]
    );
}

// ----------------------------------------------------------------------
