//! Database of a system of multiple repositories.

mod repository;

pub mod query;
pub mod text;

pub use query::{
    EagerMultiQueryDatabase, EagerMultiQueryDatabaseLatest, MemoMultiQueryDatabase,
    MemoMultiQueryDatabaseLatest, MultiQuerier, MultiQueryDatabase, MultiQueryDatabaseLatest,
};
pub use repository::RepositoryName;
pub use text::MultiTextCollection;
