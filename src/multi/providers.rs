use super::{MultiQueryDatabase, WithParsedVersion};
use crate::{
    multi::{Entries, EntriesMut, MultiQuerier, MultiQueriers, MultiQueriersMut},
    value::RepositoryName,
};
use arch_pkg_text::{
    desc::{Query, QueryMut},
    value::DependencyName,
};
use core::iter::FusedIterator;

/// [Iterator] over all immutable queriers of packages which list a certain [`DependencyName`] in their
/// [`provides`](Query::provides) array.
///
/// This iterator is created by calling [`MultiQueryDatabase::alternative_providers`].
#[derive(Debug, Clone)]
pub struct AlternativeProviders<'r, 'query, 'name, Querier> {
    target: DependencyName<'name>,
    current: Option<Entries<'r, 'query, Querier>>, // always filled if queriers is filled
    queriers: MultiQueriers<'r, 'query, Querier>,
}

impl<'r, 'query, 'name, Querier> AlternativeProviders<'r, 'query, 'name, Querier> {
    /// Create the struct in such a way to ensure invariant.
    fn new(target: DependencyName<'name>, queriers: MultiQueriers<'r, 'query, Querier>) -> Self {
        let mut result = AlternativeProviders {
            target,
            current: None,
            queriers,
        };
        result.change_querier();
        result
    }

    /// Extract an element from [`Self::queriers`] into [`Self::current`].
    fn change_querier(&mut self) {
        self.current = self.queriers.next().map(MultiQuerier::entries);
    }
}

impl<'r, 'query, 'name, Querier: Query<'r>> Iterator
    for AlternativeProviders<'r, 'query, 'name, Querier>
{
    type Item = (
        RepositoryName<'query>,
        &'r WithParsedVersion<'query, Querier>,
    );
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if cfg!(debug_assertions) && self.current.is_none() && self.queriers.next().is_some() {
                panic!("Invariant violated! `current` was emptied before `queriers`");
            }

            for (repository, querier) in self.current.as_mut()? {
                let found = querier
                    .provides()
                    .into_iter()
                    .flatten()
                    .map(|provide| provide.components())
                    .any(|(name, _)| name == self.target);
                if found {
                    return Some((repository, querier));
                }
            }

            self.change_querier();
        }
    }
}

impl<'r, 'query, 'name, Querier: Query<'r>> FusedIterator
    for AlternativeProviders<'r, 'query, 'name, Querier>
{
}

/// [Iterator] over all mutable queriers of packages which list a certain [`DependencyName`] in their
/// [`provides`](QueryMut::provides_mut) array.
///
/// This iterator is created by calling [`MultiQueryDatabase::alternative_providers_mut`].
#[derive(Debug)]
pub struct AlternativeProvidersMut<'r, 'query, 'name, Querier> {
    target: DependencyName<'name>,
    current: Option<EntriesMut<'r, 'query, Querier>>, // always filled if queriers is filled
    queriers: MultiQueriersMut<'r, 'query, Querier>,
}

impl<'r, 'query, 'name, Querier> AlternativeProvidersMut<'r, 'query, 'name, Querier> {
    /// Create the struct in such a way to ensure invariant.
    fn new(target: DependencyName<'name>, queriers: MultiQueriersMut<'r, 'query, Querier>) -> Self {
        let mut result = AlternativeProvidersMut {
            target,
            current: None,
            queriers,
        };
        result.change_querier();
        result
    }

    /// Extract an element from [`Self::queriers`] into [`Self::current`].
    fn change_querier(&mut self) {
        self.current = self.queriers.next().map(MultiQuerier::entries_mut);
    }
}

impl<'r, 'query, 'name, Querier: QueryMut<'r>> Iterator
    for AlternativeProvidersMut<'r, 'query, 'name, Querier>
{
    type Item = (
        RepositoryName<'query>,
        &'r mut WithParsedVersion<'query, Querier>,
    );
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if cfg!(debug_assertions) && self.current.is_none() && self.queriers.next().is_some() {
                panic!("Invariant violated! `current` was emptied before `queriers`");
            }

            for (repository, querier) in self.current.as_mut()? {
                let found = querier
                    .provides_mut()
                    .into_iter()
                    .flatten()
                    .map(|provide| provide.components())
                    .any(|(name, _)| name == self.target);
                if found {
                    return Some((repository, querier));
                }
            }

            self.change_querier();
        }
    }
}

impl<'r, 'query, 'name, Querier: QueryMut<'r>> FusedIterator
    for AlternativeProvidersMut<'r, 'query, 'name, Querier>
{
}

impl<'query, Querier> MultiQueryDatabase<'query, Querier> {
    /// Get an iterator over all immutable queriers of packages which list a certain [`DependencyName`] in their
    /// [`provides`](Query::provides) array.
    ///
    /// This method is prefixed with "alternative" because a package doesn't usually list itself in its own `provides`,
    /// and consequently, would usually be excluded from this iterator. Beware that if it does list itself, its own
    /// name would be included.
    pub fn alternative_providers<'r, 'name>(
        &'r self,
        target: DependencyName<'name>,
    ) -> AlternativeProviders<'r, 'query, 'name, Querier> {
        AlternativeProviders::new(target, self.queriers())
    }

    /// Get an iterator over all mutable queriers of packages which list a certain [`DependencyName`] in their
    /// [`provides`](QueryMut::provides_mut) array.
    ///
    /// This method is prefixed with "alternative" because a package doesn't usually list itself in its own `provides`,
    /// and consequently, would usually be excluded from this iterator. Beware that if it does list itself, its own
    /// name would be included.
    pub fn alternative_providers_mut<'r, 'name>(
        &'r mut self,
        target: DependencyName<'name>,
    ) -> AlternativeProvidersMut<'r, 'query, 'name, Querier> {
        AlternativeProvidersMut::new(target, self.queriers_mut())
    }
}
