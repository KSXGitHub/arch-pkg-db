//! Database of a system of multiple repositories.

mod repository;

pub mod query;

pub use query::{EagerMultiQueryDatabase, MemoMultiQueryDatabase, MultiQueryDatabase};
pub use repository::RepositoryName;
