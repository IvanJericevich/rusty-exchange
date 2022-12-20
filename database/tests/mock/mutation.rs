use database::{ClientModel, MarketModel, Mutation, Set, SubAccountModel, SubAccountStatus};
use sea_orm::prelude::*;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};

// ----------------------------------------------------------------------

#[async_std::test]
async fn clients() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![
            vec![],
            vec![ClientModel {
                id: 1,
                email: "ivanjericevich96@gmail.com".to_owned(),
                created_at: "2022-01-01T00:00:00".parse().unwrap(),
            }],
            vec![ClientModel {
                id: 1,
                email: "ivanjericevich96@gmail.com".to_owned(),
                created_at: "2022-01-01T00:00:00".parse().unwrap(),
            }],
        ])
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .into_connection();
    // Create new
    assert_eq!(
        Mutation::create_client(&db, "ivanjericevich96@gmail.com".to_owned())
            .await
            .unwrap(),
        ClientModel {
            id: 1,
            email: "ivanjericevich96@gmail.com".to_owned(),
            created_at: "2022-01-01T00:00:00".parse().unwrap(),
        }
    );
    // Create with existing
    assert_eq!(
        Mutation::create_client(&db, "ivanjericevich96@gmail.com".to_owned())
            .await
            .unwrap_err(),
        DbErr::Custom(format!(
            "Client with email ivanjericevich96@gmail.com already exists."
        ))
    );
}

// ----------------------------------------------------------------------

#[async_std::test]
async fn markets() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![
            vec![],
            vec![MarketModel {
                id: 1,
                base_currency: "BTC".to_owned(),
                quote_currency: "USD".to_owned(),
                price_increment: 0.01,
                size_increment: 0.01,
                created_at: "2022-01-01T00:00:00".parse().unwrap(),
            }],
            vec![MarketModel {
                id: 1,
                base_currency: "BTC".to_owned(),
                quote_currency: "USD".to_owned(),
                price_increment: 0.01,
                size_increment: 0.01,
                created_at: "2022-01-01T00:00:00".parse().unwrap(),
            }],
        ])
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .into_connection();
    // Create new
    assert_eq!(
        Mutation::create_market(
            &db,
            "BTC".to_owned(),
            "USD".to_owned(),
            0.01,
            0.01
        )
            .await
            .unwrap(),
        MarketModel {
            id: 1,
            base_currency: "BTC".to_owned(),
            quote_currency: "USD".to_owned(),
            price_increment: 0.01,
            size_increment: 0.01,
            created_at: "2022-01-01T00:00:00".parse().unwrap(),
        }
    );
    // Create with existing
    assert_eq!(
        Mutation::create_market(
            &db,
            "BTC".to_owned(),
            "USD".to_owned(),
            0.01,
            0.01
        )
            .await
            .unwrap_err(),
        DbErr::Custom(format!(
            "Market with symbol BTC/USD already exists."
        ))
    );
}

// ----------------------------------------------------------------------

#[async_std::test]
async fn sub_accounts() {
    let empty_sub_account_vector: Vec<SubAccountModel> = vec![];
    let empty_client_vector: Vec<ClientModel> = vec![];
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![ClientModel {
            id: 1,
            email: "ivanjericevich96@gmail.com".to_owned(),
            created_at: "2022-01-01T00:00:00".parse().unwrap(),
        }]])
        .append_query_results(vec![
            vec![],
            vec![SubAccountModel {
                id: 1,
                name: "Test".to_owned(),
                created_at: "2022-01-01T00:00:00".parse().unwrap(),
                client_id: 1,
                status: SubAccountStatus::Active,
            }],
        ])
        .append_query_results(vec![
            empty_client_vector
        ])
        .append_query_results(vec![
            empty_sub_account_vector
        ])
        .append_query_results(vec![vec![ClientModel {
            id: 1,
            email: "ivanjericevich96@gmail.com".to_owned(),
            created_at: "2022-01-01T00:00:00".parse().unwrap(),
        }]])
        .append_query_results(vec![
            vec![SubAccountModel {
                id: 1,
                name: "Test".to_owned(),
                created_at: "2022-01-01T00:00:00".parse().unwrap(),
                client_id: 1,
                status: SubAccountStatus::Active,
            }],
        ])
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .into_connection();
    // Create new
    assert_eq!(
        Mutation::create_sub_account(
            &db,
            1,
            "Test".to_owned()
        ).await.unwrap(),
        SubAccountModel {
            id: 1,
            name: "Test".to_owned(),
            created_at: "2022-01-01T00:00:00".parse().unwrap(),
            client_id: 1,
            status: SubAccountStatus::Active,
        }
    );
    // Create with non-existent client
    assert_eq!(
        Mutation::create_sub_account(
            &db,
            1,
            "Test".to_owned(),
        )
            .await
            .unwrap_err(),
        DbErr::RecordNotFound(format!(
            "Client with id 1 does not exist."
        ))
    );
    // Create with existing
    assert_eq!(
        Mutation::create_sub_account(
            &db,
            1,
            "Test".to_owned(),
        )
            .await
            .unwrap_err(),
        DbErr::Custom(format!(
            "Sub-account with name Test already exists."
        ))
    );
}