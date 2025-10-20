use super::{InsertError, MultiQueryDatabase};
use crate::multi::RepositoryName;
use arch_pkg_text::desc::{Query, QueryMut, misc::ShouldReuse};

impl<'a, Querier: ShouldReuse> MultiQueryDatabase<'a, Querier> {
    /// Extend the database with an iterator of tuples of a [`RepositoryName`] and a querier of a `desc` file.
    fn extend_with<PairIter, Insert, InsertSuccess, InsertError>(
        &mut self,
        pairs: PairIter,
        mut insert: Insert,
    ) -> Result<(), InsertError>
    where
        PairIter: IntoIterator<Item = (RepositoryName<'a>, Querier)>,
        Insert: FnMut(&mut Self, RepositoryName<'a>, Querier) -> Result<InsertSuccess, InsertError>,
    {
        let pairs = pairs.into_iter();
        let (cap, _) = pairs.size_hint();
        self.internal.reserve(cap);
        for (repo, querier) in pairs {
            insert(self, repo, querier)?;
        }
        Ok(())
    }

    /// Extend the database with an iterator of tuples of a [`RepositoryName`] and an [immutable querier](Query)
    /// of a `desc` file.
    ///
    /// Old queriers which occupied the same pair of [repository](RepositoryName) and [name](arch_pkg_text::value::Name)
    /// would be replaced.
    pub fn extend<PairIter>(&mut self, pairs: PairIter) -> Result<(), InsertError<'_>>
    where
        Querier: Query<'a>,
        PairIter: IntoIterator<Item = (RepositoryName<'a>, Querier)>,
    {
        self.extend_with(pairs, MultiQueryDatabase::insert)
    }

    /// Extend the database with an iterator of tuples of a [`RepositoryName`] and a [mutable querier](QueryMut)
    /// of a `desc` file.
    ///
    /// Old queriers which occupied the same pair of [repository](RepositoryName) and [name](arch_pkg_text::value::Name)
    /// would be replaced.
    pub fn extend_mut<PairIter>(&mut self, pairs: PairIter) -> Result<(), InsertError<'_>>
    where
        Querier: QueryMut<'a>,
        PairIter: IntoIterator<Item = (RepositoryName<'a>, Querier)>,
    {
        self.extend_with(pairs, MultiQueryDatabase::insert_mut)
    }

    /// Extend the database with an iterator of tuples of a [`RepositoryName`] and an [immutable querier](Query)
    /// of a `desc` file.
    ///
    /// An item from the iterator would replace an existing entry which occupied the same pair of [repository](RepositoryName)
    /// and [name](arch_pkg_text::value::Name) if the iterator's item has newer [version](arch_pkg_text::value::Version) than
    /// that of the existing entry.
    pub fn extend_newer<PairIter>(&mut self, pairs: PairIter) -> Result<(), InsertError<'_>>
    where
        Querier: Query<'a>,
        PairIter: IntoIterator<Item = (RepositoryName<'a>, Querier)>,
    {
        self.extend_with(pairs, MultiQueryDatabase::insert_newer)
    }

    /// Extend the database with an iterator of tuples of a [`RepositoryName`] and a [mutable querier](QueryMut)
    /// of a `desc` file.
    ///
    /// An item from the iterator would replace an existing entry which occupied the same pair of [repository](RepositoryName)
    /// and [name](arch_pkg_text::value::Name) if the iterator's item has newer [version](arch_pkg_text::value::Version) than
    /// that of the existing entry.
    pub fn extend_newer_mut<PairIter>(&mut self, pairs: PairIter) -> Result<(), InsertError<'_>>
    where
        Querier: QueryMut<'a>,
        PairIter: IntoIterator<Item = (RepositoryName<'a>, Querier)>,
    {
        self.extend_with(pairs, MultiQueryDatabase::insert_newer_mut)
    }
}
