use crate::{
    multi::{MultiQuerier, WithVersion},
    value::RepositoryName,
};
use std::{collections::hash_map::Keys, iter::FusedIterator};

/// [Iterator] over [repository names](RepositoryName) in a [`MultiQuerier`].
#[derive(Debug, Clone)]
pub struct RepositoryNames<'r, 'query, Querier> {
    internal: Keys<'r, &'query str, WithVersion<'query, Querier>>,
}

impl<'query, Querier> Iterator for RepositoryNames<'_, 'query, Querier> {
    type Item = RepositoryName<'query>;

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next().copied().map(RepositoryName)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.internal.size_hint()
    }

    fn count(self) -> usize {
        self.internal.count()
    }
}

impl<Querier> ExactSizeIterator for RepositoryNames<'_, '_, Querier> {
    fn len(&self) -> usize {
        self.internal.len()
    }
}

impl<Querier> FusedIterator for RepositoryNames<'_, '_, Querier> {}

impl<'a, Querier> MultiQuerier<'a, Querier> {
    pub fn repositories(&self) -> RepositoryNames<'_, 'a, Querier> {
        RepositoryNames {
            internal: self.internal.keys(),
        }
    }
}
