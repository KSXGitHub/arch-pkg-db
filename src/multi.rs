//! Database of a system of multiple repositories.

pub mod query;

pub use query::{
    EagerMultiQueryDatabase, EagerMultiQueryDatabaseLatest, MemoMultiQueryDatabase,
    MemoMultiQueryDatabaseLatest, MultiQuerier, MultiQueryDatabase, MultiQueryDatabaseLatest,
};
