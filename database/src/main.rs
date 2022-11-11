use futures::executor::block_on;

use sea_orm::{DbErr};
use sea_orm_migration::prelude::*;

use database::{Migrator, Engine};

async fn run() -> Result<(), DbErr> { // Run migrations
  let db = Engine::connect().await?;

  let schema_manager = SchemaManager::new(&db); // To investigate the schema

  Migrator::refresh(&db).await?; // Run migrator
  assert!(schema_manager.has_table("post").await?); // Check if successful

  Ok(())
}

#[async_std::main]
async fn main() {
  block_on(run()).unwrap(); // Block async function; panic if unsuccessful
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_connection() { // Test that a connection to the database can be established
    block_on(Engine::connect()).unwrap();
  }
}