use crate::QueryDatabase;
use arch_pkg_text::value::Name;
use std::{collections::hash_map, iter::FusedIterator};

/// [Iterator] over all pairs of [package names](Name) and owned queriers from a [`QueryDatabase`].
#[derive(Debug)]
pub struct OwnedEntries<'name, Querier> {
    internal: hash_map::IntoIter<&'name str, Querier>,
}

impl<'name, Querier> Iterator for OwnedEntries<'name, Querier> {
    type Item = (Name<'name>, Querier);
    fn next(&mut self) -> Option<Self::Item> {
        let (name, querier) = self.internal.next()?;
        Some((Name(name), querier))
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
    type Item = (Name<'a>, Querier);
    fn into_iter(self) -> Self::IntoIter {
        OwnedEntries {
            internal: self.internal.into_iter(),
        }
    }
}
