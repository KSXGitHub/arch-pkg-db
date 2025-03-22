use super::{MultiQuerier, MultiQueryDatabase};
use crate::multi::RepositoryName;
use arch_pkg_text::{
    desc::{ParsedField, Query, QueryMut, misc::ReuseAdvice},
    value::{Name, ParsedVersion, Version},
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
    pub fn get(&self, repository: RepositoryName) -> Option<&Querier> {
        self.internal.get(repository.as_str())
    }

    /// Get a mutable reference to a querier by repository name.
    pub fn get_mut(&mut self, repository: RepositoryName) -> Option<&mut Querier> {
        self.internal.get_mut(repository.as_str())
    }
}

/// Return type of [`MultiQuerier::latest`] and [`MultiQuerier::latest_mut`].
#[derive(Debug, Clone, Copy)]
pub struct LatestQuerier<'repo, 'ver, Querier> {
    pub repository: RepositoryName<'repo>,
    pub version: Version<'ver>,
    pub parsed_version: ParsedVersion<'ver>,
    pub querier: Querier,
}

impl<'a, Querier: Query<'a>> Query<'a> for LatestQuerier<'_, '_, Querier> {
    fn query_raw_text(&self, field: ParsedField) -> Option<&'a str> {
        self.querier.query_raw_text(field)
    }
}

impl<'a, Querier: QueryMut<'a>> QueryMut<'a> for LatestQuerier<'_, '_, Querier> {
    fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
        self.querier.query_raw_text_mut(field)
    }
}

impl<Querier: ReuseAdvice> ReuseAdvice for LatestQuerier<'_, '_, Querier> {
    type ShouldReuse = Querier::ShouldReuse;
}

impl<Querier> MultiQuerier<'_, Querier> {
    /// Get an immutable reference to a querier whose package's version is greatest.
    pub fn latest<'ver>(&self) -> Option<LatestQuerier<'_, 'ver, &Querier>>
    where
        Querier: Query<'ver>,
    {
        self.internal
            .iter()
            .filter_map(|(repository, querier)| {
                let repository = RepositoryName(repository);
                let version = querier.version()?;
                let parsed_version = version.parse().ok()?;
                Some(LatestQuerier {
                    repository,
                    version,
                    parsed_version,
                    querier,
                })
            })
            .max_by_key(|querier| querier.parsed_version)
    }

    /// Get a mutable reference to a querier whose package's version is greatest.
    pub fn latest_mut<'ver>(&mut self) -> Option<LatestQuerier<'_, 'ver, &mut Querier>>
    where
        Querier: QueryMut<'ver>,
    {
        self.internal
            .iter_mut()
            .filter_map(|(repository, querier)| {
                let repository = RepositoryName(repository);
                let version = querier.version_mut()?;
                let parsed_version = version.parse().ok()?;
                Some(LatestQuerier {
                    repository,
                    version,
                    parsed_version,
                    querier,
                })
            })
            .max_by_key(|querier| querier.parsed_version)
    }
}
