//! Database of a single repository.

pub mod query;
pub mod text;

pub use query::{EagerQueryDatabase, MemoQueryDatabase, QueryDatabase};
pub use text::TextCollection;
