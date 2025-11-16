use crate::single::{InsertError, InsertNewerError, QueryDatabase};
use arch_pkg_text::{
    desc::{Query, QueryMut},
    misc::desc::ShouldReuse,
};

impl<'a, Querier: ShouldReuse> QueryDatabase<'a, Querier> {
    /// Construct a database with an iterator of queriers of `desc` files.
    fn from_queriers_with<QuerierIter, Insert, InsertSuccess, InsertError>(
        queriers: QuerierIter,
        mut insert: Insert,
    ) -> Result<Self, InsertError>
    where
        QuerierIter: IntoIterator<Item = Querier>,
        Insert: FnMut(&mut Self, Querier) -> Result<InsertSuccess, InsertError>,
    {
        let queriers = queriers.into_iter();
        let (cap, _) = queriers.size_hint();
        let mut db = QueryDatabase::with_capacity(cap);
        for querier in queriers {
            insert(&mut db, querier)?;
        }
        Ok(db)
    }

    /// Construct a database with an iterator of [immutable queriers](Query) of `desc` files.
    ///
    /// If there are collisions between queriers in [name](arch_pkg_text::value::Name), the later querier would override the earlier.
    pub fn from_queriers<QuerierIter>(queriers: QuerierIter) -> Result<Self, InsertError>
    where
        Querier: Query<'a>,
        QuerierIter: IntoIterator<Item = Querier>,
    {
        QueryDatabase::from_queriers_with(queriers, QueryDatabase::insert)
    }

    /// Construct a database with an iterator of [mutable queriers](QueryMut) of `desc` files.
    ///
    /// If there are collisions between queriers in [name](arch_pkg_text::value::Name), the later querier would override the earlier.
    pub fn from_queriers_mut<QuerierIter>(queriers: QuerierIter) -> Result<Self, InsertError>
    where
        Querier: QueryMut<'a>,
        QuerierIter: IntoIterator<Item = Querier>,
    {
        QueryDatabase::from_queriers_with(queriers, QueryDatabase::insert_mut)
    }

    /// Construct a database from an iterator of [immutable queriers](Query) of `desc` files.
    ///
    /// If there are collisions between queriers in [name](arch_pkg_text::value::Name), the one with newer
    /// [package version](arch_pkg_text::value::Version) would override the older.
    pub fn from_newer_queriers<QuerierIter>(
        queriers: QuerierIter,
    ) -> Result<Self, InsertNewerError<'a>>
    where
        Querier: Query<'a>,
        QuerierIter: IntoIterator<Item = Querier>,
    {
        QueryDatabase::from_queriers_with(queriers, QueryDatabase::insert_newer)
    }

    /// Construct a database from an iterator of [mutable queriers](QueryMut) of `desc` files.
    ///
    /// If there are collisions between queriers in [name](arch_pkg_text::value::Name), the one with newer
    /// [package version](arch_pkg_text::value::Version) would override the older.
    pub fn from_newer_queriers_mut<QuerierIter>(
        queriers: QuerierIter,
    ) -> Result<Self, InsertNewerError<'a>>
    where
        Querier: QueryMut<'a>,
        QuerierIter: IntoIterator<Item = Querier>,
    {
        QueryDatabase::from_queriers_with(queriers, QueryDatabase::insert_newer_mut)
    }
}
