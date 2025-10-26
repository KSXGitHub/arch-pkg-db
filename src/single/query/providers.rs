use super::QueryDatabase;
use crate::single::query::{Queriers, QueriersMut};
use arch_pkg_text::{
    desc::{Query, QueryMut},
    value::DependencyName,
};
use core::iter::FusedIterator;

/// [Iterator] over all immutable queriers of packages which list a certain [`DependencyName`] in their
/// [`provides`](Query::provides) array.
///
/// This iterator is created by calling [`QueryDatabase::alternative_providers`].
#[derive(Debug, Clone)]
pub struct AlternativeProviders<'r, 'name, Querier> {
    target: DependencyName<'name>,
    queriers: Queriers<'r, 'name, Querier>,
}

impl<'r, 'name, Querier: Query<'r>> Iterator for AlternativeProviders<'r, 'name, Querier> {
    type Item = &'r Querier;

    fn next(&mut self) -> Option<Self::Item> {
        self.queriers.find(|querier| {
            querier
                .provides()
                .into_iter()
                .flatten()
                .map(|provide| provide.components())
                .any(|(name, _)| name == self.target)
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, max) = self.queriers.size_hint();
        (0, max)
    }
}

impl<'r, 'name, Querier: Query<'r>> FusedIterator for AlternativeProviders<'r, 'name, Querier> {}

/// [Iterator] over all mutable queriers of packages which list a certain [`DependencyName`] in their
/// [`provides`](QueryMut::provides_mut) array.
///
/// This iterator is created by calling [`QueryDatabase::alternative_providers_mut`].
#[derive(Debug)]
pub struct AlternativeProvidersMut<'r, 'name, Querier> {
    target: DependencyName<'name>,
    queriers: QueriersMut<'r, 'name, Querier>,
}

impl<'r, 'name, Querier: QueryMut<'r>> Iterator for AlternativeProvidersMut<'r, 'name, Querier> {
    type Item = &'r mut Querier;

    fn next(&mut self) -> Option<Self::Item> {
        for querier in self.queriers.by_ref() {
            let found = querier
                .provides_mut()
                .into_iter()
                .flatten()
                .map(|provide| provide.components())
                .any(|(name, _)| name == self.target);
            if found {
                return Some(querier);
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, max) = self.queriers.size_hint();
        (0, max)
    }
}

impl<'r, 'name, Querier: QueryMut<'r>> FusedIterator
    for AlternativeProvidersMut<'r, 'name, Querier>
{
}

impl<Querier> QueryDatabase<'_, Querier> {
    /// Get an iterator over all immutable queriers of packages which list a certain [`DependencyName`] in their
    /// [`provides`](Query::provides) array.
    ///
    /// This method is prefixed with "alternative" because a package doesn't usually list itself in its own `provides`,
    /// and consequently, would usually be excluded from this iterator. Beware that if it does list itself, its own
    /// name would be included.
    pub fn alternative_providers<'r: 'name, 'name>(
        &'r self,
        target: DependencyName<'name>,
    ) -> AlternativeProviders<'r, 'name, Querier>
    where
        Querier: Query<'r>,
    {
        let queriers = self.queriers();
        AlternativeProviders { target, queriers }
    }

    /// Get an iterator over all mutable queriers of packages which list a certain [`DependencyName`] in their
    /// [`provides`](QueryMut::provides_mut) array.
    ///
    /// This method is prefixed with "alternative" because a package doesn't usually list itself in its own `provides`,
    /// and consequently, would usually be excluded from this iterator. Beware that if it does list itself, its own
    /// name would be included.
    pub fn alternative_providers_mut<'r: 'name, 'name>(
        &'r mut self,
        target: DependencyName<'name>,
    ) -> AlternativeProvidersMut<'r, 'name, Querier>
    where
        Querier: QueryMut<'r>,
    {
        let queriers = self.queriers_mut();
        AlternativeProvidersMut { target, queriers }
    }
}
