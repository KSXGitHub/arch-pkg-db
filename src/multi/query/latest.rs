use super::{MultiQuerier, WithRepository, WithVersion};
use crate::{
    misc::{AttachedUtils, IntoAttached},
    multi::RepositoryName,
};
use arch_pkg_text::desc::{Query, QueryMut};

/// Return type of [`MultiQuerier::latest`] and [`MultiQuerier::latest_mut`].
pub type LatestQuerier<'a, Querier> = WithRepository<'a, WithVersion<'a, Querier>>;

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
