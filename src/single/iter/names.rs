use crate::QueryDatabase;
use arch_pkg_text::value::Name;
use core::iter::FusedIterator;
use std::collections::hash_map::Keys;

/// [Iterator] over all [package names](Name) in a [`QueryDatabase`].
#[derive(Debug, Clone)]
pub struct Names<'r, 'name, Querier> {
    internal: Keys<'r, &'name str, Querier>,
}

impl<'name, Querier> Iterator for Names<'_, 'name, Querier> {
    type Item = Name<'name>;

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

impl<'a, Querier> QueryDatabase<'a, Querier> {
    /// Get an iterator over all [package names](Name).
    pub fn names(&self) -> Names<'_, 'a, Querier> {
        Names {
            internal: self.internal.keys(),
        }
    }
}
