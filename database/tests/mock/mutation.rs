use database::{ActiveClientModel, ClientModel, Mutation, Set};
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
