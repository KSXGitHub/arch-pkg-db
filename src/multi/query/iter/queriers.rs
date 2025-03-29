use crate::{
    MultiQueryDatabase,
    multi::{MultiQuerier, query::WithVersion},
};
use core::iter::FusedIterator;
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
