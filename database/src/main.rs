use database::Migrator;
use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    cli::run_cli(Migrator).await;
}

#[cfg(test)]
mod tests {
    use database::{Engine, Query};

    use futures::executor::block_on;

    #[test]
    fn test_connection() {
        // Test that a connection to the database can be established
        let db = block_on(Engine::connect()).unwrap();

        let x = block_on(Query::find_orders(
            &db,
            None,
            None,
            Some("Test".to_owned()),
            None,
            None,
            None,
            None,
            Some(1),
            Some(1),
        ))
        .unwrap();

        println!("{:?}", x)
    }
}
