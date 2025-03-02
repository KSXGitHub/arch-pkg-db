mod add;
mod db;
mod get;
mod new;

pub use add::{InsertError, NoNameError};
pub use db::LookupError;

use arch_pkg_text::desc::{EagerQuerier, MemoQuerier};
use std::collections::HashMap;

/// Database to lookup queriers from their package names.
#[derive(Debug, Clone)]
pub struct QueryDatabase<'a, Querier> {
    /// Map package names to their queriers.
    internal: HashMap<&'a str, Querier>,
}

/// Database to lookup eager queriers from their package names.
pub type EagerQueryDatabase<'a> = QueryDatabase<'a, EagerQuerier<'a>>;

/// Database to lookup memo queriers from their package names.
pub type MemoQueryDatabase<'a> = QueryDatabase<'a, MemoQuerier<'a>>;
