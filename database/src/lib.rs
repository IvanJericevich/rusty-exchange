// Bring modules into scope
mod core; // Export core SQL queries/mutations
mod entities; // Export models
mod migrator; // Export migrator - one may want to run migrations in an API on start-up

// Export desired modules
pub use crate::core::*;
pub use crate::entities::*;
pub use crate::migrator::*;