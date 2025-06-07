use super::Entry;
use crate::QueryDatabase;
use core::iter::FusedIterator;
use std::collections::hash_map;

/// [Iterator] over all pairs of [package names](Name) and immutable queriers in a [`QueryDatabase`].
#[derive(Debug, Clone)]
pub struct Entries<'r, 'name, Querier> {
    internal: hash_map::Iter<'r, &'name str, Querier>,
}

impl<'r, 'name, Querier> Iterator for Entries<'r, 'name, Querier> {
    type Item = Entry<'name, &'r Querier>;

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
    type Item = Entry<'name, &'r mut Querier>;

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
