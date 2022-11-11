use dotenv::dotenv;

use std::env;

use sea_orm::{DatabaseConnection, DbErr, Database};

pub struct Engine;

impl Engine {
  pub async fn connect() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok(); // Load the environment variables from the ".env" file
  
    // Get credentials from environment
    let db_url = env::var("DB_URL").unwrap_or_else(|_e| {
      let name = env::var("DB_NAME").expect("DB_NAME environment variable not found");
      let host = env::var("DB_HOST").unwrap_or("localhost".to_owned());
      let password = env::var("DB_PASSWORD").expect("DB_PASSWORD environment variable not found");
      let username = env::var("DB_USERNAME").unwrap_or("postgres".to_owned());
      format!("postgresql://{}:{}@{}:5432/{}", username, password, host, name)
    });
  
    Database::connect(db_url).await // Create database connection pool
  }
}