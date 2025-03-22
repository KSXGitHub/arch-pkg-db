use super::{MultiQuerier, MultiQueryDatabase};
use crate::multi::RepositoryName;
use arch_pkg_text::{
    desc::{Query, QueryMut},
    value::{Name, Version},
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

impl<'a, Querier> MultiQuerier<'a, Querier> {
    /// Get an immutable reference to a querier by repository name.
    pub fn get(&self, repository: RepositoryName) -> Option<&Querier> {
        self.internal.get(repository.as_str())
    }

    /// Get a mutable reference to a querier by repository name.
    pub fn get_mut(&mut self, repository: RepositoryName) -> Option<&mut Querier> {
        self.internal.get_mut(repository.as_str())
    }

    /// Get an immutable reference to a querier whose package's version is greatest.
    pub fn latest(&self) -> Option<(Version<'a>, &Querier)>
    where
        Querier: Query<'a>,
    {
        todo!("implement vercmp: https://man.archlinux.org/man/vercmp.8.en")
    }

    /// Get a mutable reference to a querier whose package's version is greatest.
    pub fn latest_mut(&mut self) -> Option<(Version<'a>, &mut Querier)>
    where
        Querier: QueryMut<'a>,
    {
        todo!("implement vercmp: https://man.archlinux.org/man/vercmp.8.en")
    }
}
