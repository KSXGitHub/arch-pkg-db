use super::{
    IntoWithRepositoryName, LatestQuerier, MultiQuerier, MultiQueryDatabase,
    MultiQueryDatabaseLatest, WithParsedVersionUtils,
};
use crate::value::RepositoryName;

impl<'a, Querier> MultiQuerier<'a, Querier> {
    /// Get an immutable reference to a querier whose package's version is greatest.
    pub fn latest(&self) -> Option<LatestQuerier<'a, &Querier>> {
        self.internal
            .iter()
            .max_by_key(|(_, querier)| querier.parsed_version())
            .map(|(repository, querier)| {
                querier
                    .to_ref()
                    .with_repository_name(RepositoryName(repository))
            })
    }

    /// Get a mutable reference to a querier whose package's version is greatest.
    pub fn latest_mut(&mut self) -> Option<LatestQuerier<'a, &mut Querier>> {
        self.internal
            .iter_mut()
            .max_by_key(|(_, querier)| querier.parsed_version())
            .map(|(repository, querier)| {
                querier
                    .to_ref_mut()
                    .with_repository_name(RepositoryName(repository))
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
