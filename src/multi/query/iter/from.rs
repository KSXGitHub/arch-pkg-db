use crate::multi::{
    RepositoryName,
    query::{InsertError, MultiQueryDatabase},
};
use arch_pkg_text::{
    desc::{Query, QueryMut},
    misc::desc::ShouldReuse,
};

impl<'a, Querier: ShouldReuse> MultiQueryDatabase<'a, Querier> {
    /// Construct a database with an iterator of tuples of a [`RepositoryName`] and a querier of a `desc` file.
    fn from_queriers_with<PairIter, Insert, InsertSuccess, InsertError>(
        pairs: PairIter,
        mut insert: Insert,
    ) -> Result<Self, InsertError>
    where
        PairIter: IntoIterator<Item = (RepositoryName<'a>, Querier)>,
        Insert: FnMut(&mut Self, RepositoryName<'a>, Querier) -> Result<InsertSuccess, InsertError>,
    {
        let pairs = pairs.into_iter();
        let (cap, _) = pairs.size_hint();
        let mut db = MultiQueryDatabase::with_capacity(cap);
        for (repo, querier) in pairs {
            insert(&mut db, repo, querier)?;
        }
        Ok(db)
    }

    /// Construct a database with an iterator of tuples of a [`RepositoryName`] and an [immutable querier](Query)
    /// of a `desc` file.
    ///
    /// If there are collisions between pairs of [repository](RepositoryName) and [name](arch_pkg_text::value::Name),
    /// the later pair would override the earlier.
    pub fn from_queriers<PairIter>(pairs: PairIter) -> Result<Self, InsertError<'a>>
    where
        Querier: Query<'a>,
        PairIter: IntoIterator<Item = (RepositoryName<'a>, Querier)>,
    {
        MultiQueryDatabase::from_queriers_with(pairs, MultiQueryDatabase::insert)
    }

    /// Construct a database with an iterator of tuples of a [`RepositoryName`] and a [mutable querier](QueryMut)
    /// of a `desc` file.
    ///
    /// If there are collisions between pairs of [repository](RepositoryName) and [name](arch_pkg_text::value::Name),
    /// the later pair would override the earlier.
    pub fn from_queriers_mut<PairIter>(pairs: PairIter) -> Result<Self, InsertError<'a>>
    where
        Querier: QueryMut<'a>,
        PairIter: IntoIterator<Item = (RepositoryName<'a>, Querier)>,
    {
        MultiQueryDatabase::from_queriers_with(pairs, MultiQueryDatabase::insert_mut)
    }

    /// Construct a database with an iterator of tuples of a [`RepositoryName`] and an [immutable querier](Query)
    /// of a `desc` file.
    ///
    /// If there are collisions between pairs of [repository](RepositoryName) and [name](arch_pkg_text::value::Name),
    /// the one with newer [package version](arch_pkg_text::value::Version) would override the older.
    pub fn from_newer_queriers<PairIter>(pairs: PairIter) -> Result<Self, InsertError<'a>>
    where
        Querier: Query<'a>,
        PairIter: IntoIterator<Item = (RepositoryName<'a>, Querier)>,
    {
        MultiQueryDatabase::from_queriers_with(pairs, MultiQueryDatabase::insert_newer)
    }

    /// Construct a database with an iterator of tuples of a [`RepositoryName`] and a [mutable querier](QueryMut)
    /// of a `desc` file.
    ///
    /// If there are collisions between pairs of [repository](RepositoryName) and [name](arch_pkg_text::value::Name),
    /// the one with newer [package version](arch_pkg_text::value::Version) would override the older.
    pub fn from_newer_queriers_mut<PairIter>(pairs: PairIter) -> Result<Self, InsertError<'a>>
    where
        Querier: QueryMut<'a>,
        PairIter: IntoIterator<Item = (RepositoryName<'a>, Querier)>,
    {
        MultiQueryDatabase::from_queriers_with(pairs, MultiQueryDatabase::insert_newer_mut)
    }
}
