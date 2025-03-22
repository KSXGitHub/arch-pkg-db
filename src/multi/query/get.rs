use super::{MultiQuerier, MultiQueryDatabase, WithRepository, WithVersion};
use crate::{
    misc::{AttachedUtils, IntoAttached},
    multi::RepositoryName,
};
use arch_pkg_text::{
    desc::{Query, QueryMut},
    value::Name,
};

impl<'a, Querier> MultiQueryDatabase<'a, Querier> {
    /// Get an immutable reference to a set of queriers of packages from different repositories
    /// by package name.
    pub fn get(&self, name: Name) -> Option<&MultiQuerier<'a, Querier>> {
        self.internal.get(name.as_str())
    }

    /// Get a mutable reference to a set of queriers of packages from different repositories
    /// by package name.
    pub fn get_mut(&mut self, name: Name) -> Option<&mut MultiQuerier<'a, Querier>> {
        self.internal.get_mut(name.as_str())
    }
}

impl<Querier> MultiQuerier<'_, Querier> {
    /// Get an immutable reference to a querier by repository name.
    pub fn get(&self, repository: RepositoryName) -> Option<WithVersion<&Querier>> {
        self.internal
            .get(repository.as_str())
            .map(AttachedUtils::as_deref)
            .map(AttachedUtils::copied_attachment)
    }

    /// Get a mutable reference to a querier by repository name.
    pub fn get_mut(&mut self, repository: RepositoryName) -> Option<WithVersion<&mut Querier>> {
        self.internal
            .get_mut(repository.as_str())
            .map(AttachedUtils::as_deref_mut)
            .map(AttachedUtils::copied_attachment)
    }
}

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
