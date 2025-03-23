use super::{MultiQuerier, MultiQueryDatabase, WithVersion};
use crate::{misc::AttachedUtils, multi::RepositoryName};
use arch_pkg_text::value::Name;

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
