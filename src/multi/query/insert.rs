use super::{MultiQueryDatabase, WithVersion};
use crate::{misc::IntoAttached, multi::RepositoryName};
use arch_pkg_text::{
    desc::{Query, QueryMut, misc::ShouldReuse},
    value::{Name, ParseVersionError, Version},
};
use derive_more::{Display, Error};
use pipe_trait::Pipe;

/// Error type of [`MultiQueryDatabase::insert`] and [`MultiQueryDatabase::insert_mut`].
#[derive(Debug, Display, Error)]
pub enum InsertError<'a> {
    #[display("Querier does not provide a name")]
    NoName,
    #[display("Querier does not provide a version")]
    NoVersion,
    #[display("Version provided by the querier has invalid syntax: {_0}")]
    ParseVersion(#[error(not(source))] ParseVersionError<'a>),
}

impl<'a, Querier: ShouldReuse> MultiQueryDatabase<'a, Querier> {
    /// Add a querier of a `desc` file to the database.
    ///
    /// If an older querier already occupied the same pair of [name] and [repository], it will be returned inside `Ok(Some(_))`.
    ///
    /// [name]: arch_pkg_text::value::Name
    /// [repository]: RepositoryName
    fn insert_with<GetName, GetVersion>(
        &mut self,
        repository: RepositoryName<'a>,
        mut querier: Querier,
        get_name: GetName,
        get_version: GetVersion,
    ) -> Result<Option<WithVersion<'a, Querier>>, InsertError<'a>>
    where
        GetName: FnOnce(&mut Querier) -> Option<Name<'a>>,
        GetVersion: FnOnce(&mut Querier) -> Option<Version<'a>>,
    {
        let name = get_name(&mut querier).ok_or(InsertError::NoName)?;
        let version = querier
            .pipe_mut(get_version)
            .ok_or(InsertError::NoVersion)?
            .parse()
            .map_err(InsertError::ParseVersion)?;
        self.internal
            .entry(&name)
            .or_default()
            .internal
            .insert(&repository, querier.into_attached(version))
            .pipe(Ok)
    }

    /// Add an [immutable querier](Query) of a `desc` file to the database.
    ///
    /// If an older querier already occupied the same pair of [name] and [repository], it will be returned inside `Ok(Some(_))`.
    ///
    /// [name]: arch_pkg_text::value::Name
    /// [repository]: RepositoryName
    pub fn insert(
        &mut self,
        repository: RepositoryName<'a>,
        querier: Querier,
    ) -> Result<Option<WithVersion<'a, Querier>>, InsertError<'a>>
    where
        Querier: Query<'a>,
    {
        self.insert_with(
            repository,
            querier,
            |querier| querier.name(),
            |querier| querier.version(),
        )
    }

    /// Add a [mutable querier](QueryMut) of a `desc` file to the database.
    ///
    /// If an older querier already occupied the same pair of [name] and [repository], it will be returned inside `Ok(Some(_))`.
    ///
    /// [name]: arch_pkg_text::value::Name
    /// [repository]: RepositoryName
    pub fn insert_mut(
        &mut self,
        repository: RepositoryName<'a>,
        querier: Querier,
    ) -> Result<Option<WithVersion<'a, Querier>>, InsertError<'a>>
    where
        Querier: QueryMut<'a>,
    {
        self.insert_with(repository, querier, Querier::name_mut, Querier::version_mut)
    }
}
