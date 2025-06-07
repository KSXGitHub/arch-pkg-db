use super::Entry;
use crate::QueryDatabase;
use core::iter::FusedIterator;
use std::collections::hash_map;

/// [Iterator] over all pairs of [package names](Name) and owned queriers from a [`QueryDatabase`].
#[derive(Debug)]
pub struct OwnedEntries<'name, Querier> {
    internal: hash_map::IntoIter<&'name str, Querier>,
}

impl<'name, Querier> Iterator for OwnedEntries<'name, Querier> {
    type Item = Entry<'name, Querier>;

    fn next(&mut self) -> Option<Self::Item> {
        let (name, querier) = self.internal.next()?;
        Some(Entry::new_unchecked(name, querier))
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

impl<'a, Querier> IntoIterator for QueryDatabase<'a, Querier> {
    type IntoIter = OwnedEntries<'a, Querier>;
    type Item = Entry<'a, Querier>;
    fn into_iter(self) -> Self::IntoIter {
        OwnedEntries {
            internal: self.internal.into_iter(),
        }
    }
}
