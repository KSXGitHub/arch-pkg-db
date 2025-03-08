use crate::{IterEntries, IterEntriesMut, QueryDatabase};
use arch_pkg_text::value::Name;
use core::iter::FusedIterator;
use std::collections::hash_map;

/// [Iterator] over all pairs of [package names](Name) and immutable queriers in a [`QueryDatabase`].
#[derive(Debug, Clone)]
pub struct Entries<'r, 'name, Querier> {
    internal: hash_map::Iter<'r, &'name str, Querier>,
}

impl<'r, 'name, Querier> Iterator for Entries<'r, 'name, Querier> {
    type Item = (Name<'name>, &'r Querier);
    fn next(&mut self) -> Option<Self::Item> {
        let (name, querier) = self.internal.next()?;
        Some((Name(name), querier))
    }
}

impl<Querier> ExactSizeIterator for Entries<'_, '_, Querier> {
    fn len(&self) -> usize {
        self.internal.len()
    }
}

impl<Querier> FusedIterator for Entries<'_, '_, Querier> {}

/// [Iterator] over all pairs of [package names](Name) and mutable queriers in a [`QueryDatabase`].
#[derive(Debug)]
pub struct EntriesMut<'r, 'name, Querier> {
    internal: hash_map::IterMut<'r, &'name str, Querier>,
}

impl<'r, 'name, Querier> Iterator for EntriesMut<'r, 'name, Querier> {
    type Item = (Name<'name>, &'r mut Querier);
    fn next(&mut self) -> Option<Self::Item> {
        let (name, querier) = self.internal.next()?;
        Some((Name(name), querier))
    }
}

impl<Querier> ExactSizeIterator for EntriesMut<'_, '_, Querier> {
    fn len(&self) -> usize {
        self.internal.len()
    }
}

impl<Querier> FusedIterator for EntriesMut<'_, '_, Querier> {}

impl<'a, Querier> QueryDatabase<'a, Querier> {
    /// Get an iterator over all pairs of [package names](Name) and immutable queriers.
    pub fn entries(&self) -> Entries<'_, 'a, Querier> {
        Entries {
            internal: self.internal.iter(),
        }
    }

    /// Get an iterator over all pairs of [package names](Name) and mutable queriers.
    pub fn entries_mut(&mut self) -> EntriesMut<'_, 'a, Querier> {
        EntriesMut {
            internal: self.internal.iter_mut(),
        }
    }
}

impl<Querier> IterEntries for QueryDatabase<'_, Querier> {
    fn entries(&self) -> impl Iterator<Item = (Name, &Self::Querier)> {
        self.entries()
    }
}

impl<Querier> IterEntriesMut for QueryDatabase<'_, Querier> {
    fn entries_mut(&mut self) -> impl Iterator<Item = (Name, &mut Self::Querier)> {
        self.entries_mut()
    }
}
