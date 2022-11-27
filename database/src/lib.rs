// Bring modules into scope
mod core; // Export core SQL queries/mutations
mod entities; // Do not export entities - re-export them in the "models" module
pub mod migrator; // Export migrator - one may want to run migrations in an API on start-up
pub mod models; // Export models

// Export required modules
pub use crate::core::*;
pub use migrator::*;
pub use models::*;
