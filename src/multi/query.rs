mod insert;
mod new;

use arch_pkg_text::desc::{EagerQuerier, MemoQuerier};
use std::collections::HashMap;

/// Queriers of multiple same-name packages from different repositories.
#[derive(Debug, Clone)]
pub struct MultiQuerier<'a, Querier> {
    /// Map repository names to their queriers.
    internal: HashMap<&'a str, Querier>,
}

/// Database to lookup queriers from their package names and repositories.
#[derive(Debug, Clone)]
pub struct MultiQueryDatabase<'a, Querier> {
    /// Map package names to their multi-queriers.
    internal: HashMap<&'a str, MultiQuerier<'a, Querier>>,
}

/// Database to lookup eager queriers from their package names.
pub type EagerMultiQueryDatabase<'a> = MultiQueryDatabase<'a, EagerQuerier<'a>>;

/// Database to lookup memo queriers from their package names.
pub type MemoMultiQueryDatabase<'a> = MultiQueryDatabase<'a, MemoQuerier<'a>>;
