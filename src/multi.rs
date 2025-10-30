//! Database of a system of multiple repositories.

pub mod query;
pub mod text;

pub use query::{
    EagerMultiQueryDatabase, EagerMultiQueryDatabaseLatest, MemoMultiQueryDatabase,
    MemoMultiQueryDatabaseLatest, MultiQuerier, MultiQueryDatabase, MultiQueryDatabaseLatest,
};
pub use text::MultiTextCollection;
