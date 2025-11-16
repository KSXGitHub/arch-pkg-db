//! Database of a system of multiple repositories.

mod combine;
mod extend;
mod get;
mod insert;
mod iter;
mod latest;
mod misc;
mod new;
mod providers;

pub use combine::{
    IntoWithParsedVersion, IntoWithRepositoryName, WithParsedVersion, WithParsedVersionUtils,
    WithRepositoryName, WithRepositoryNameUtils,
};
pub use insert::{InsertError, InsertNewerReturn};
pub use iter::{
    Entries, EntriesMut, LatestEntries, LatestEntriesMut, LatestQueriers, LatestQueriersMut,
    MultiEntries, MultiEntriesMut, MultiOwnedEntries, MultiQueriers, MultiQueriersMut, Names,
    OwnedEntries, Queriers, QueriersMut, RepositoryNames,
};
pub use providers::{AlternativeProviders, AlternativeProvidersMut};

use arch_pkg_text::desc::{EagerQuerier, MemoQuerier};
use std::collections::HashMap;

/// Return type of [`MultiQuerier::latest`] and [`MultiQuerier::latest_mut`].
type LatestQuerier<'a, Querier> = WithRepositoryName<'a, WithParsedVersion<'a, Querier>>;

/// Queriers of multiple same-name packages from different repositories.
#[derive(Debug, Clone)]
pub struct MultiQuerier<'a, Querier> {
    /// Map repository names to their queriers.
    internal: HashMap<&'a str, WithParsedVersion<'a, Querier>>,
}

/// Database to lookup queriers from their package names and repositories.
///
/// This type of database is designed for repository-aware use cases.
/// For example: Multiple repositories whose packages may have duplicated names.
#[derive(Debug, Clone)]
pub struct MultiQueryDatabase<'a, Querier> {
    /// Map package names to their multi-queriers.
    internal: HashMap<&'a str, MultiQuerier<'a, Querier>>,
}

/// Database view to lookup queriers of the latest packages from their names.
#[derive(Debug, Clone, Copy)]
pub struct MultiQueryDatabaseLatest<Ref> {
    base: Ref,
}

/// Database to lookup eager queriers from their package names.
pub type EagerMultiQueryDatabase<'a> = MultiQueryDatabase<'a, EagerQuerier<'a>>;

/// Database to lookup memo queriers from their package names.
pub type MemoMultiQueryDatabase<'a> = MultiQueryDatabase<'a, MemoQuerier<'a>>;

/// Database to lookup eager queriers of the latest packages from their names.
pub type EagerMultiQueryDatabaseLatest<'r, 'a> =
    MultiQueryDatabaseLatest<&'r EagerMultiQueryDatabase<'a>>;

/// Database to lookup memo queriers of the latest packages from their names.
pub type MemoMultiQueryDatabaseLatest<'r, 'a> =
    MultiQueryDatabaseLatest<&'r mut MemoMultiQueryDatabase<'a>>;
