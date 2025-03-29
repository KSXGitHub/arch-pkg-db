//! Database of a system of multiple repositories.

mod repository;

pub mod query;

pub use query::{
    EagerMultiQueryDatabase, EagerMultiQueryDatabaseLatest, MemoMultiQueryDatabase,
    MemoMultiQueryDatabaseLatest, MultiQuerier, MultiQueryDatabase, MultiQueryDatabaseLatest,
};
pub use repository::RepositoryName;
