use crate::multi::{MultiQuerier, MultiQueryDatabase, MultiQueryDatabaseLatest};
use arch_pkg_text::value::Name;
use core::iter::FusedIterator;
use core::ops::Deref;
use std::collections::hash_map::Keys;

/// [Iterator] over all [package names](Name) in a [`MultiQueryDatabase`].
#[derive(Debug, Clone)]
pub struct Names<'r, 'query, Querier> {
    internal: Keys<'r, &'query str, MultiQuerier<'query, Querier>>,
}

impl<'query, Querier> Iterator for Names<'_, 'query, Querier> {
    type Item = Name<'query>;

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next().copied().map(Name)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.internal.size_hint()
    }

    fn count(self) -> usize {
        self.internal.count()
    }
}

impl<Querier> ExactSizeIterator for Names<'_, '_, Querier> {
    fn len(&self) -> usize {
        self.internal.len()
    }
}

impl<Querier> FusedIterator for Names<'_, '_, Querier> {}

impl<'a, Querier> MultiQueryDatabase<'a, Querier> {
    /// Get an iterator over all [package names](Name).
    pub fn names(&self) -> Names<'_, 'a, Querier> {
        Names {
            internal: self.internal.keys(),
        }
    }
}

impl<Ref> MultiQueryDatabaseLatest<Ref> {
    /// Get an iterator over all [package names](Name).
    pub fn names<'a, Querier>(&self) -> Names<'_, 'a, Querier>
    where
        Ref: Deref<Target = MultiQueryDatabase<'a, Querier>>,
    {
        self.base.names()
    }
}
