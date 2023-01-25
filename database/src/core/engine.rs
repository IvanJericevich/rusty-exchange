use dotenv::dotenv;

use std::env;

use sea_orm::{Database, DatabaseConnection, DbErr};

pub struct Engine;

impl Engine {
    pub async fn connect() -> Result<DatabaseConnection, DbErr> {
        dotenv().ok(); // Load the environment variables from the ".env" file

        // Get credentials from environment
        let db_url = env::var("POSTGRES_URL").unwrap_or_else(|_| {
            let name = env::var("POSTGRES_DB").expect("POSTGRES_DB environment variable not found");
            let host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_owned());
            let password =
                env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD environment variable not found");
            let username = env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_owned());
            format!(
                "postgresql://{}:{}@{}:5432/{}",
                username, password, host, name
            )
        });

        Database::connect(db_url).await // Create database connection pool
    }
}
