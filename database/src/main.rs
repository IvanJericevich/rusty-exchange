use database::Migrator;
use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    cli::run_cli(Migrator).await;
}

#[cfg(test)]
mod tests {
    use database::Engine;

    use futures::executor::block_on;

    #[test]
    fn test_connection() {
        // Test that a connection to the database can be established
        block_on(Engine::connect()).unwrap();
    }
}