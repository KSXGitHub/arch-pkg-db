//! Databases from single archives.

pub mod query;
pub mod text;

pub use query::{EagerQueryDatabase, MemoQueryDatabase, QueryDatabase};
pub use text::{Text, TextCollection};
