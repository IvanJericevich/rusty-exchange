use futures::executor::block_on;

use sea_orm::DbErr;
use sea_orm_migration::prelude::*;

use database::{Engine, Migrator};

async fn run() -> Result<(), DbErr> {
    // Run migrations
    let db = Engine::connect().await?;

    // let schema_manager = SchemaManager::new(&db); // To investigate the schema

    // Migrator::down(&db, Some(1)).await?; // Run migrator
    // Migrator::refresh(&db).await?;
    // assert!(schema_manager.has_table("post").await?); // Check if successful

    Ok(())
}

#[async_std::main]
async fn main() {
    block_on(run()).unwrap(); // Block async function; panic if unsuccessful
}


