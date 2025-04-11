use crate::multi::{
    MultiQuerier, MultiQueryDatabase, MultiQueryDatabaseLatest, RepositoryName,
    query::{LatestQuerier, WithVersion},
};
use arch_pkg_text::desc::{Query, QueryMut};
use arch_pkg_text::value::Name;
use core::{
    iter::FusedIterator,
    ops::{Deref, DerefMut},
};
use pipe_trait::Pipe;
use std::collections::hash_map;

/// [Iterator] over all pairs of [package names](Name) and immutable queriers in a [`MultiQueryDatabase`].
#[derive(Debug, Clone)]
pub struct MultiEntries<'r, 'query, Querier> {
    internal: hash_map::Iter<'r, &'query str, MultiQuerier<'query, Querier>>,
}

impl<'r, 'query, Querier> Iterator for MultiEntries<'r, 'query, Querier> {
    type Item = (Name<'query>, &'r MultiQuerier<'query, Querier>);

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

impl<Querier> ExactSizeIterator for MultiEntries<'_, '_, Querier> {
    fn len(&self) -> usize {
        self.internal.len()
    }
}

impl<Querier> FusedIterator for MultiEntries<'_, '_, Querier> {}

/// [Iterator] over all pairs of [package names](Name) and mutable queriers in a [`MultiQueryDatabase`].
#[derive(Debug)]
pub struct MultiEntriesMut<'r, 'query, Querier> {
    internal: hash_map::IterMut<'r, &'query str, MultiQuerier<'query, Querier>>,
}

impl<'r, 'query, Querier> Iterator for MultiEntriesMut<'r, 'query, Querier> {
    type Item = (Name<'query>, &'r mut MultiQuerier<'query, Querier>);

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

impl<Querier> ExactSizeIterator for MultiEntriesMut<'_, '_, Querier> {
    fn len(&self) -> usize {
        self.internal.len()
    }
}

impl<Querier> FusedIterator for MultiEntriesMut<'_, '_, Querier> {}

impl<'a, Querier> MultiQueryDatabase<'a, Querier> {
    /// Get an iterator over all pairs of [package names](Name) and immutable queriers.
    pub fn entries(&self) -> MultiEntries<'_, 'a, Querier> {
        MultiEntries {
            internal: self.internal.iter(),
        }
    }

    /// Get an iterator over all pairs of [package names](Name) and mutable queriers.
    pub fn entries_mut(&mut self) -> MultiEntriesMut<'_, 'a, Querier> {
        MultiEntriesMut {
            internal: self.internal.iter_mut(),
        }
    }

    /// Get an iterator over all pairs of [package names](Name) and immutable queriers of
    /// the latest versions of each package.
    pub fn latest_entries(&self) -> LatestEntries<'_, 'a, Querier> {
        LatestEntries {
            internal: self.internal.iter(),
        }
    }

    /// Get an iterator over all pairs of [package names](Name) and mutable queriers of
    /// the latest versions of each package.
    pub fn latest_entries_mut(&mut self) -> LatestEntriesMut<'_, 'a, Querier> {
        LatestEntriesMut {
            internal: self.internal.iter_mut(),
        }
    }
}

/// [Iterator] over all pairs of [repository names](RepositoryName) and immutable queriers in a [`MultiQuerier`].
#[derive(Debug, Clone)]
pub struct Entries<'r, 'query, Querier> {
    internal: hash_map::Iter<'r, &'query str, WithVersion<'query, Querier>>,
}

impl<'r, 'query, Querier> Iterator for Entries<'r, 'query, Querier> {
    type Item = (RepositoryName<'query>, &'r Querier);

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

impl<Querier> ExactSizeIterator for Entries<'_, '_, Querier> {
    fn len(&self) -> usize {
        self.internal.len()
    }
}

impl<Querier> FusedIterator for Entries<'_, '_, Querier> {}

/// [Iterator] over all pairs of [repository names](RepositoryName) and mutable queriers in a [`MultiQuerier`].
#[derive(Debug)]
pub struct EntriesMut<'r, 'query, Querier> {
    internal: hash_map::IterMut<'r, &'query str, WithVersion<'query, Querier>>,
}

impl<'r, 'query, Querier> Iterator for EntriesMut<'r, 'query, Querier> {
    type Item = (RepositoryName<'query>, &'r mut WithVersion<'query, Querier>);

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

impl<Querier> ExactSizeIterator for EntriesMut<'_, '_, Querier> {
    fn len(&self) -> usize {
        self.internal.len()
    }
}

impl<Querier> FusedIterator for EntriesMut<'_, '_, Querier> {}

impl<'a, Querier> MultiQuerier<'a, Querier> {
    /// Get an iterator over all pairs of [repository names](RepositoryName) and immutable queriers.
    pub fn entries(&self) -> Entries<'_, 'a, Querier> {
        Entries {
            internal: self.internal.iter(),
        }
    }

    /// Get an iterator over all pairs of [repository names](RepositoryName) and mutable queriers.
    pub fn entries_mut(&mut self) -> EntriesMut<'_, 'a, Querier> {
        EntriesMut {
            internal: self.internal.iter_mut(),
        }
    }
}

/// [Iterator] over all pairs of [package names](Name) and immutable queriers in [`MultiQueryDatabaseLatest`].
#[derive(Debug, Clone)]
pub struct LatestEntries<'r, 'query, Querier> {
    internal: hash_map::Iter<'r, &'query str, MultiQuerier<'query, Querier>>,
}

impl<'r, 'query, Querier: Query<'query>> Iterator for LatestEntries<'r, 'query, Querier> {
    type Item = (Name<'query>, LatestQuerier<'query, &'r Querier>);

    fn next(&mut self) -> Option<Self::Item> {
        let (name, querier) = self.internal.next()?;
        let name = Name(name);
        let querier = querier.latest()?;
        Some((name, querier))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.internal.len().pipe(Some))
    }

    fn count(self) -> usize {
        self.internal.count()
    }
}

impl<'query, Querier: Query<'query>> FusedIterator for LatestEntries<'_, 'query, Querier> {}

/// [Iterator] over all pairs of [package names](Name) and mutable queriers in [`MultiQueryDatabaseLatest`].
#[derive(Debug)]
pub struct LatestEntriesMut<'r, 'query, Querier> {
    internal: hash_map::IterMut<'r, &'query str, MultiQuerier<'query, Querier>>,
}

impl<'r, 'query, Querier: QueryMut<'query>> Iterator for LatestEntriesMut<'r, 'query, Querier> {
    type Item = (Name<'query>, LatestQuerier<'query, &'r mut Querier>);

    fn next(&mut self) -> Option<Self::Item> {
        let (name, querier) = self.internal.next()?;
        let name = Name(name);
        let querier = querier.latest_mut()?;
        Some((name, querier))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.internal.len().pipe(Some))
    }

    fn count(self) -> usize {
        self.internal.count()
    }
}

impl<'query, Querier: QueryMut<'query>> FusedIterator for LatestEntriesMut<'_, 'query, Querier> {}

impl<Ref> MultiQueryDatabaseLatest<Ref> {
    /// Get an iterator over all pairs of [package names](Name) and immutable queriers.
    pub fn entries<'query, Querier>(&self) -> LatestEntries<'_, 'query, Querier>
    where
        Ref: Deref<Target = MultiQueryDatabase<'query, Querier>>,
    {
        self.base.latest_entries()
    }

    /// Get an iterator over all pairs of [package names](Name) and mutable queriers.
    pub fn entries_mut<'query, Querier>(&mut self) -> LatestEntriesMut<'_, 'query, Querier>
    where
        Ref: DerefMut<Target = MultiQueryDatabase<'query, Querier>>,
    {
        self.base.latest_entries_mut()
    }
}
