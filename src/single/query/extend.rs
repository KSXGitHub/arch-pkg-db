use super::{InsertError, QueryDatabase};
use arch_pkg_text::{
    desc::{Query, QueryMut},
    misc::desc::ShouldReuse,
};

impl<'a, Querier: ShouldReuse> QueryDatabase<'a, Querier> {
    /// Extend the database with an iterator of queriers of `desc` files.
    ///
    /// Old queriers which occupied the same [name](arch_pkg_text::value::Name) as some of the new queriers would be replaced.
    fn extend_with<QuerierIter, Insert, InsertSuccess>(
        &mut self,
        queriers: QuerierIter,
        mut insert: Insert,
    ) -> Result<(), InsertError>
    where
        QuerierIter: IntoIterator<Item = Querier>,
        Insert: FnMut(&mut Self, Querier) -> Result<InsertSuccess, InsertError>,
    {
        let queriers = queriers.into_iter();
        if let (_, Some(cap)) = queriers.size_hint() {
            self.internal.reserve(cap);
        }
        for querier in queriers {
            insert(self, querier)?;
        }
        Ok(())
    }

    /// Extend the database with an iterator of [immutable queriers](Query) of `desc` files.
    ///
    /// Old queriers which occupied the same [name](arch_pkg_text::value::Name) as some of the new queriers would be replaced.
    pub fn extend<QuerierIter>(&mut self, queriers: QuerierIter) -> Result<(), InsertError>
    where
        Querier: Query<'a>,
        QuerierIter: IntoIterator<Item = Querier>,
    {
        self.extend_with(queriers, QueryDatabase::insert)
    }

    /// Extend the database with an iterator of [mutable queriers](QueryMut) of `desc` files.
    ///
    /// Old queriers which occupied the same [name](arch_pkg_text::value::Name) as some of the new queriers would be replaced.
    pub fn extend_mut<QuerierIter>(&mut self, queriers: QuerierIter) -> Result<(), InsertError>
    where
        Querier: QueryMut<'a>,
        QuerierIter: IntoIterator<Item = Querier>,
    {
        self.extend_with(queriers, QueryDatabase::insert_mut)
    }
}
