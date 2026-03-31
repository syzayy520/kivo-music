//! Library database (SQLite) foundation.
//!
//! This module is intentionally small and declarative:
//! - `open_db(app)` resolves the AppData path, opens the connection, applies pragmas, and ensures schema.
//! - `schema` contains the SQL and migration/ensure logic.
//!
//! P0 scope: schema + open + migrate only (no scanning/indexing logic here).

pub mod errors;
pub mod open;
pub(crate) mod scan;
pub mod schema;

pub use open::{open_db, open_db_at_path};
