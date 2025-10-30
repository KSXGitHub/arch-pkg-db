use super::{LatestQuerier, MultiQuerier, MultiQueryDatabase, MultiQueryDatabaseLatest};
use crate::{
    misc::{AttachedUtils, IntoAttached},
    value::RepositoryName,
};
use arch_pkg_text::desc::{Query, QueryMut};

impl<'a, Querier> MultiQuerier<'a, Querier> {
    /// Get an immutable reference to a querier whose package's version is greatest.
    pub fn latest<'query>(&self) -> Option<LatestQuerier<'a, &Querier>>
    where
        Querier: Query<'query>,
    {
        self.internal
            .iter()
            .max_by_key(|(_, querier)| querier.attachment())
            .map(|(repository, querier)| {
                querier
                    .as_deref()
                    .copied_attachment()
                    .into_attached(RepositoryName(repository))
            })
    }

    /// Get a mutable reference to a querier whose package's version is greatest.
    pub fn latest_mut<'query>(&mut self) -> Option<LatestQuerier<'a, &mut Querier>>
    where
        Querier: QueryMut<'query>,
    {
        self.internal
            .iter_mut()
            .max_by_key(|(_, querier)| *querier.attachment())
            .map(|(repository, querier)| {
                querier
                    .as_deref_mut()
                    .copied_attachment()
                    .into_attached(RepositoryName(repository))
            })
    }
}

impl<Querier> MultiQueryDatabase<'_, Querier> {
    /// Combine the different repositories into a database view of immutable queriers
    /// that lookup the latest versions of packages.
    pub fn latest(&self) -> MultiQueryDatabaseLatest<&Self> {
        MultiQueryDatabaseLatest { base: self }
    }

    /// Combine the different repositories into a database view of mutable queriers
    /// that lookup the latest versions of packages.
    pub fn latest_mut(&mut self) -> MultiQueryDatabaseLatest<&mut Self> {
        MultiQueryDatabaseLatest { base: self }
    }
}
