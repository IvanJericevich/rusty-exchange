// TODO: Make query arguments more concise (e.g. use models)
// TODO: Think about using &str so we dont use clone() all the time
// TODO: having vs filter
// TODO: should models implement display (for logging and errors)
// TODO: create custom active model
// Bring modules into scope
mod core; // Export core SQL queries/mutations
mod entities; // Do not export entities - re-export them in the "models" module
mod migrator; // Export migrator - one may want to run migrations in an API on start-up
mod models; // Export models

// Export required modules
pub use crate::core::*;
pub use crate::migrator::*;
pub use crate::models::*;
