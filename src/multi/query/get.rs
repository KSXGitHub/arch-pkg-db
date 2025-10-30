use super::{
    LatestQuerier, MultiQuerier, MultiQueryDatabase, MultiQueryDatabaseLatest, WithVersion,
};
use crate::{desc::value::RepositoryName, misc::AttachedUtils};
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
    pub fn get(&self, repository: RepositoryName) -> Option<WithVersion<'_, &Querier>> {
        self.internal
            .get(repository.as_str())
            .map(AttachedUtils::as_deref)
            .map(AttachedUtils::copied_attachment)
    }

    /// Get a mutable reference to a querier by repository name.
    pub fn get_mut(&mut self, repository: RepositoryName) -> Option<WithVersion<'_, &mut Querier>> {
        self.internal
            .get_mut(repository.as_str())
            .map(AttachedUtils::as_deref_mut)
            .map(AttachedUtils::copied_attachment)
    }
}

impl<'r, 'a, Querier> MultiQueryDatabaseLatest<&'r MultiQueryDatabase<'a, Querier>> {
    /// Get an immutable reference to the querier of the latest version of a package by its name.
    pub fn get<'query>(&self, name: Name) -> Option<LatestQuerier<'a, &'r Querier>>
    where
        Querier: Query<'query>,
    {
        self.base.get(name)?.latest()
    }
}

impl<'a, Querier> MultiQueryDatabaseLatest<&mut MultiQueryDatabase<'a, Querier>> {
    /// Get an mutable reference to the querier of the latest version of a package by its name.
    pub fn get_mut<'query>(&mut self, name: Name) -> Option<LatestQuerier<'a, &mut Querier>>
    where
        Querier: QueryMut<'query>,
    {
        self.base.get_mut(name)?.latest_mut()
    }
}
