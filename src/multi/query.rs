mod get;
mod insert;
mod latest;
mod new;

use super::RepositoryName;
use crate::misc::Attached;
use arch_pkg_text::{
    desc::{EagerQuerier, MemoQuerier},
    value::ParsedVersion,
};
use std::collections::HashMap;

/// Querier attached to a version.
type WithVersion<'a, Querier> = Attached<Querier, ParsedVersion<'a>>;

/// Querier attached to a repository name.
type WithRepository<'a, Querier> = Attached<Querier, RepositoryName<'a>>;

/// Queriers of multiple same-name packages from different repositories.
#[derive(Debug, Clone)]
pub struct MultiQuerier<'a, Querier> {
    /// Map repository names to their queriers.
    internal: HashMap<&'a str, WithVersion<'a, Querier>>,
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
