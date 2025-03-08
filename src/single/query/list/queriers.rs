use crate::{IterQueriers, IterQueriersMut, QueryDatabase};
use core::iter::FusedIterator;
use std::collections::hash_map::{Values, ValuesMut};

/// [Iterator] over all immutable queriers in a [`QueryDatabase`].
#[derive(Debug, Clone)]
pub struct Queriers<'r, 'name, Querier> {
    internal: Values<'r, &'name str, Querier>,
}

impl<'r, Querier> Iterator for Queriers<'r, '_, Querier> {
    type Item = &'r Querier;
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

/// [Iterator] over all mutable queriers in a [`QueryDatabase`].
#[derive(Debug)]
pub struct QueriersMut<'r, 'name, Querier> {
    internal: ValuesMut<'r, &'name str, Querier>,
}

impl<'r, Querier> Iterator for QueriersMut<'r, '_, Querier> {
    type Item = &'r mut Querier;
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

impl<'a, Querier> QueryDatabase<'a, Querier> {
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

impl<Querier> IterQueriers for QueryDatabase<'_, Querier> {
    fn queriers(&self) -> impl Iterator<Item = &Self::Querier> {
        self.queriers()
    }
}

impl<Querier> IterQueriersMut for QueryDatabase<'_, Querier> {
    fn queriers_mut(&mut self) -> impl Iterator<Item = &mut Self::Querier> {
        self.queriers_mut()
    }
}
