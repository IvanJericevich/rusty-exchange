// TODO: Think about using &str so we dont use clone() all the time
// TODO: having vs filter
// TODO: should models implement display (for logging and errors)
// TODO: Use system time instead of datetime
// Bring modules into scope
mod core; // Export core SQL queries/mutations
mod entities; // Do not export entities - re-export them in the "models" module
mod migrator; // Export migrator - one may want to run migrations in an API on start-up

// Export required modules
pub use crate::core::*;
pub use crate::migrator::*;
pub use crate::entities::{clients, markets, orders, fills, sea_orm_active_enums::*, sub_accounts, positions};

// Re-export sea-orm functionality
pub use sea_orm::ActiveValue::Set;
pub use sea_orm::{Database, DatabaseConnection, DbErr};

// Re-export utoipa functionality
pub use utoipa;
