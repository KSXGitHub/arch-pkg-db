use crate::{
    desc::value::RepositoryName,
    multi::{MultiQuerier, MultiQueryDatabase, query::WithVersion},
};
use arch_pkg_text::value::Name;
use core::iter::FusedIterator;
use std::collections::hash_map;

/// [Iterator] over all pairs of [package names](Name) and owned queriers from a [`MultiQueryDatabase`].
#[derive(Debug)]
pub struct MultiOwnedEntries<'query, Querier> {
    internal: hash_map::IntoIter<&'query str, MultiQuerier<'query, Querier>>,
}

impl<'query, Querier> Iterator for MultiOwnedEntries<'query, Querier> {
    type Item = (Name<'query>, MultiQuerier<'query, Querier>);

    fn next(&mut self) -> Option<Self::Item> {
        let (name, querier) = self.internal.next()?;
        Some((Name(name), querier))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.internal.size_hint()
    }

    fn count(self) -> usize {
        self.internal.count()
    }
}

impl<Querier> ExactSizeIterator for MultiOwnedEntries<'_, Querier> {
    fn len(&self) -> usize {
        self.internal.len()
    }
}

impl<Querier> FusedIterator for MultiOwnedEntries<'_, Querier> {}

impl<'a, Querier> IntoIterator for MultiQueryDatabase<'a, Querier> {
    type IntoIter = MultiOwnedEntries<'a, Querier>;
    type Item = (Name<'a>, MultiQuerier<'a, Querier>);
    fn into_iter(self) -> Self::IntoIter {
        MultiOwnedEntries {
            internal: self.internal.into_iter(),
        }
    }
}

/// [Iterator] over all pairs of [repository names](RepositoryName) and owned queriers from a [`MultiQuerier`].
#[derive(Debug)]
pub struct OwnedEntries<'query, Querier> {
    internal: hash_map::IntoIter<&'query str, WithVersion<'query, Querier>>,
}

impl<'query, Querier> Iterator for OwnedEntries<'query, Querier> {
    type Item = (RepositoryName<'query>, WithVersion<'query, Querier>);

    fn next(&mut self) -> Option<Self::Item> {
        let (name, querier) = self.internal.next()?;
        Some((RepositoryName(name), querier))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.internal.size_hint()
    }

    fn count(self) -> usize {
        self.internal.count()
    }
}

impl<Querier> ExactSizeIterator for OwnedEntries<'_, Querier> {
    fn len(&self) -> usize {
        self.internal.len()
    }
}

impl<Querier> FusedIterator for OwnedEntries<'_, Querier> {}

impl<'a, Querier> IntoIterator for MultiQuerier<'a, Querier> {
    type IntoIter = OwnedEntries<'a, Querier>;
    type Item = (RepositoryName<'a>, WithVersion<'a, Querier>);
    fn into_iter(self) -> Self::IntoIter {
        OwnedEntries {
            internal: self.internal.into_iter(),
        }
    }
}
