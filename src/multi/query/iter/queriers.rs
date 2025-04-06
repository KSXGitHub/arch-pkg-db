use crate::multi::{
    MultiQuerier, MultiQueryDatabase, MultiQueryDatabaseLatest,
    query::{LatestQuerier, WithVersion},
};
use arch_pkg_text::desc::Query;
use core::{iter::FusedIterator, ops::Deref};
use pipe_trait::Pipe;
use std::collections::hash_map::{Values, ValuesMut};

/// [Iterator] over all immutable queriers in a [`MultiQueryDatabase`].
#[derive(Debug, Clone)]
pub struct MultiQueriers<'r, 'query, Querier> {
    internal: Values<'r, &'query str, MultiQuerier<'query, Querier>>,
}

impl<'r, 'query, Querier> Iterator for MultiQueriers<'r, 'query, Querier> {
    type Item = &'r MultiQuerier<'query, Querier>;

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.internal.size_hint()
    }

    fn count(self) -> usize {
        self.internal.count()
    }
}

impl<Querier> ExactSizeIterator for MultiQueriers<'_, '_, Querier> {
    fn len(&self) -> usize {
        self.internal.len()
    }
}

impl<Querier> FusedIterator for MultiQueriers<'_, '_, Querier> {}

/// [Iterator] over all mutable queriers in a [`MultiQueryDatabase`].
#[derive(Debug)]
pub struct MultiQueriersMut<'r, 'query, Querier> {
    internal: ValuesMut<'r, &'query str, MultiQuerier<'query, Querier>>,
}

impl<'r, 'query, Querier> Iterator for MultiQueriersMut<'r, 'query, Querier> {
    type Item = &'r mut MultiQuerier<'query, Querier>;

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.internal.size_hint()
    }

    fn count(self) -> usize {
        self.internal.count()
    }
}

impl<Querier> ExactSizeIterator for MultiQueriersMut<'_, '_, Querier> {
    fn len(&self) -> usize {
        self.internal.len()
    }
}

impl<Querier> FusedIterator for MultiQueriersMut<'_, '_, Querier> {}

impl<'a, Querier> MultiQueryDatabase<'a, Querier> {
    /// Get an iterator over all immutable queriers.
    pub fn queriers(&self) -> MultiQueriers<'_, 'a, Querier> {
        MultiQueriers {
            internal: self.internal.values(),
        }
    }

    /// Get an iterator over all mutable queriers.
    pub fn queriers_mut(&mut self) -> MultiQueriersMut<'_, 'a, Querier> {
        MultiQueriersMut {
            internal: self.internal.values_mut(),
        }
    }
}

/// [Iterator] over all immutable queriers in a [`MultiQuerier`].
#[derive(Debug, Clone)]
pub struct Queriers<'r, 'query, Querier> {
    internal: Values<'r, &'query str, WithVersion<'query, Querier>>,
}

impl<'r, 'query, Querier> Iterator for Queriers<'r, 'query, Querier> {
    type Item = &'r WithVersion<'query, Querier>;

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.internal.size_hint()
    }

    fn count(self) -> usize {
        self.internal.count()
    }
}

impl<Querier> ExactSizeIterator for Queriers<'_, '_, Querier> {
    fn len(&self) -> usize {
        self.internal.len()
    }
}

impl<Querier> FusedIterator for Queriers<'_, '_, Querier> {}

/// [Iterator] over all mutable queriers in a [`MultiQuerier`].
#[derive(Debug)]
pub struct QueriersMut<'r, 'query, Querier> {
    internal: ValuesMut<'r, &'query str, WithVersion<'query, Querier>>,
}

impl<'r, 'query, Querier> Iterator for QueriersMut<'r, 'query, Querier> {
    type Item = &'r mut WithVersion<'query, Querier>;

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.internal.size_hint()
    }

    fn count(self) -> usize {
        self.internal.count()
    }
}

impl<Querier> ExactSizeIterator for QueriersMut<'_, '_, Querier> {
    fn len(&self) -> usize {
        self.internal.len()
    }
}

impl<Querier> FusedIterator for QueriersMut<'_, '_, Querier> {}

impl<'a, Querier> MultiQuerier<'a, Querier> {
    /// Get an iterator over all immutable queriers.
    pub fn queriers(&self) -> Queriers<'_, 'a, Querier> {
        Queriers {
            internal: self.internal.values(),
        }
    }

    /// Get an iterator over all mutable queriers.
    pub fn queriers_mut(&mut self) -> QueriersMut<'_, 'a, Querier> {
        QueriersMut {
            internal: self.internal.values_mut(),
        }
    }
}

/// [Iterator] over all immutable queriers in a [`MultiQueryDatabaseLatest`].
#[derive(Debug, Clone)]
pub struct LatestQueriers<'r, 'query, Querier> {
    internal: Values<'r, &'query str, MultiQuerier<'query, Querier>>,
}

impl<'r, 'query, Querier: Query<'query>> Iterator for LatestQueriers<'r, 'query, Querier> {
    type Item = LatestQuerier<'query, &'r Querier>;

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next()?.latest()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.internal.len().pipe(Some))
    }

    fn count(self) -> usize {
        self.internal.count()
    }
}

impl<'query, Querier: Query<'query>> FusedIterator for LatestQueriers<'_, 'query, Querier> {}

/// [Iterator] over all mutable queriers in a [`MultiQueryDatabaseLatest`].
#[derive(Debug)]
pub struct LatestQueriersMut<'r, 'query, Querier> {
    internal: ValuesMut<'r, &'query str, MultiQuerier<'query, Querier>>,
}

impl<'r, 'query, Querier: QueryMut<'query>> Iterator for LatestQueriersMut<'r, 'query, Querier> {
    type Item = LatestQuerier<'query, &'r mut Querier>;

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next()?.latest_mut()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.internal.len().pipe(Some))
    }

    fn count(self) -> usize {
        self.internal.count()
    }
}

impl<'query, Querier: QueryMut<'query>> FusedIterator for LatestQueriersMut<'_, 'query, Querier> {}

impl<Ref> MultiQueryDatabaseLatest<Ref> {
    /// Get an iterator over all immutable queriers.
    pub fn queriers<'query, Querier>(&self) -> LatestQueriers<'_, 'query, Querier>
    where
        Ref: Deref<Target = MultiQueryDatabase<'query, Querier>>,
    {
        LatestQueriers {
            internal: self.base.internal.values(),
        }
    }

    /// Get an iterator over all mutable queriers.
    pub fn queriers_mut<'query, Querier>(&mut self) -> LatestQueriersMut<'_, 'query, Querier>
    where
        Ref: DerefMut<Target = MultiQueryDatabase<'query, Querier>>,
    {
        LatestQueriersMut {
            internal: self.base.internal.values_mut(),
        }
    }
}
