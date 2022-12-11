// TODO: Implement more precise pagination for orders and positions (e.g. regex for order client_id, datetime, etc.)
// TODO: OrderSide should just be Side
// TODO: Remove foreign keys from custom joins
// TODO: Make query arguments more concise (e.g. use models)
// TODO: Add quote_size, base_size, quote_price, base_price
// TODO: Make client_id a string and rename to client_order_id
// TODO: Create active/inactive status for client
// TODO: Create active/inactive status for sub_account
// TODO: Make order_id a uuid
// Bring modules into scope
mod core; // Export core SQL queries/mutations
mod entities; // Do not export entities - re-export them in the "models" module
pub mod migrator; // Export migrator - one may want to run migrations in an API on start-up
pub mod models; // Export models

// Export required modules
pub use crate::core::*;
pub use migrator::*;
pub use models::*;
