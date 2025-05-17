use crate::single::query::{InsertError, QueryDatabase};
use arch_pkg_text::{
    desc::{Query, QueryMut},
    misc::desc::ShouldReuse,
};

impl<'a, Querier: ShouldReuse> QueryDatabase<'a, Querier> {
    /// Construct the database with an iterator of queriers of `desc` files.
    ///
    /// If there are collision between queriers regarding [name](arch_pkg_text::value::Name), the later querier would override the earlier.
    fn from_queriers_with<QuerierIter, Insert, InsertSuccess>(
        queriers: QuerierIter,
        mut insert: Insert,
    ) -> Result<Self, InsertError>
    where
        QuerierIter: IntoIterator<Item = Querier>,
        Insert: FnMut(&mut Self, Querier) -> Result<InsertSuccess, InsertError>,
    {
        let queriers = queriers.into_iter();
        let mut db = match queriers.size_hint() {
            (_, Some(cap)) => QueryDatabase::with_capacity(cap),
            (_, None) => QueryDatabase::new(),
        };
        for querier in queriers {
            insert(&mut db, querier)?;
        }
        Ok(db)
    }

    /// Construct the database with an iterator of [immutable queriers](Query) of `desc` files.
    ///
    /// If there are collision between queriers regarding [name](arch_pkg_text::value::Name), the later querier would override the earlier.
    pub fn from_queriers<QuerierIter>(queriers: QuerierIter) -> Result<Self, InsertError>
    where
        Querier: Query<'a>,
        QuerierIter: IntoIterator<Item = Querier>,
    {
        QueryDatabase::from_queriers_with(queriers, QueryDatabase::insert)
    }

    /// Construct the database with an iterator of [mutable queriers](QueryMut) of `desc` files.
    ///
    /// If there are collision between queriers regarding [name](arch_pkg_text::value::Name), the later querier would override the earlier.
    pub fn from_queriers_mut<QuerierIter>(queriers: QuerierIter) -> Result<Self, InsertError>
    where
        Querier: QueryMut<'a>,
        QuerierIter: IntoIterator<Item = Querier>,
    {
        QueryDatabase::from_queriers_with(queriers, QueryDatabase::insert_mut)
    }
}
