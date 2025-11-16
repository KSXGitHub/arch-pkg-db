//! Database of a single repository.

mod extend;
mod get;
mod insert;
mod iter;
mod misc;
mod new;
mod providers;

pub use insert::{InsertError, InsertNewerError, InsertNewerReturn};
pub use iter::{Entries, EntriesMut, Entry, Names, OwnedEntries, Queriers, QueriersMut};
pub use providers::{AlternativeProviders, AlternativeProvidersMut};

use arch_pkg_text::desc::{EagerQuerier, MemoQuerier};
use std::collections::HashMap;

/// Database to lookup queriers from their package names.
///
/// This type of database is designed for repository-unaware use cases such as:
/// * Local packages: Packages that have been installed into the system.
/// * Single repository archive.
/// * Merging multiple repository archives where duplications don't exist.
#[derive(Debug, Clone)]
pub struct QueryDatabase<'a, Querier> {
    /// Map package names to their queriers.
    internal: HashMap<&'a str, Querier>,
}

/// Database to lookup eager queriers from their package names.
pub type EagerQueryDatabase<'a> = QueryDatabase<'a, EagerQuerier<'a>>;

/// Database to lookup memo queriers from their package names.
pub type MemoQueryDatabase<'a> = QueryDatabase<'a, MemoQuerier<'a>>;
