use database::{clients, markets, sub_accounts, orders, sea_orm_active_enums::{OrderSide, OrderType, OrderStatus}};
use sea_orm::*;

#[cfg(feature = "mock")]
pub fn prepare_mock_db() -> DatabaseConnection {
    MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![
            vec![clients::Model {
                id: 1,
                email: "ivanjericevich96@gmail.com".to_owned(),
                created_at: "2022-01-01T00:00:00".parse().unwrap(),
            }],
            vec![clients::Model {
                id: 1,
                email: "ivanjericevich96@gmail.com".to_owned(),
                created_at: "2022-01-01T00:00:00".parse().unwrap(),
            }],
        ])
        .append_query_results(vec![
            vec![markets::Model {
                id: 1,
                base_currency: "BTC".to_owned(),
                quote_currency: "USD".to_owned(),
                price_increment: 0.01,
                size_increment: 0.01,
                created_at: "2022-01-01T00:00:00".parse().unwrap(),
            }],
            vec![markets::Model {
                id: 1,
                base_currency: "BTC".to_owned(),
                quote_currency: "USD".to_owned(),
                price_increment: 0.01,
                size_increment: 0.01,
                created_at: "2022-01-01T00:00:00".parse().unwrap(),
            }],
        ])
        .append_query_results(vec![
            vec![sub_accounts::Model {
                id: 1,
                name: "Test".to_owned(),
                created_at: "2022-01-01T00:00:00".parse().unwrap(),
                client_id: 1,
            }],
        ])
        .append_query_results(vec![
            vec![(
                clients::Model {
                    id: 1,
                    email: "ivanjericevich96@gmail.com".to_owned(),
                    created_at: "2022-01-01T00:00:00".parse().unwrap(),
                },
                sub_accounts::Model {
                    id: 1,
                    name: "Test".to_owned(),
                    created_at: "2022-01-01T00:00:00".parse().unwrap(),
                    client_id: 1,
                }
            )],
        ])
        // .append_query_results(vec![
        //     vec![
        //         sub_accounts::Model {
        //             id: 1,
        //             name: "Test".to_owned(),
        //             created_at: "2022-01-01T00:00:00".parse().unwrap(),
        //             client_id: 1,
        //         }
        //     ],
        // ])
        // .append_query_results(vec![
        //     vec![
        //         markets::Model {
        //             id: 1,
        //             base_currency: "BTC".to_owned(),
        //             quote_currency: "USD".to_owned(),
        //             price_increment: 0.01,
        //             size_increment: 0.01,
        //             created_at: "2022-01-01T00:00:00".parse().unwrap(),
        //         }
        //     ]
        // ])
        // .append_exec_results(vec![
        //     MockExecResult {
        //         last_insert_id: 6,
        //         rows_affected: 1,
        //     },
        //     MockExecResult {
        //         last_insert_id: 6,
        //         rows_affected: 5,
        //     },
        // ])
        .into_connection()
}
