use super::{MultiQueryDatabase, WithVersion};
use crate::{
    misc::{Attached, AttachedUtils, IntoAttached},
    multi::RepositoryName,
};
use arch_pkg_text::{
    desc::{Query, QueryMut, misc::ShouldReuse},
    value::{Name, ParseVersionError, ParsedVersion, Version},
};
use core::mem::replace;
use derive_more::{Display, Error};
use pipe_trait::Pipe;

/// Error type of [`MultiQueryDatabase::insert`] and [`MultiQueryDatabase::insert_mut`].
#[derive(Debug, Display, Clone, Error)]
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

/// Return type of [`MultiQueryDatabase::insert_newer`] and [`MultiQueryDatabase::insert_newer_mut`] upon success.
#[derive(Debug, Clone, Copy)]
pub enum InsertNewerReturn<'a, Querier> {
    /// The entry was unoccupied, the querier was successfully inserted.
    Unoccupied,
    /// The entry was occupied by a querier whose package version is older than the provided querier.
    /// The provided querier thus replaced the old one.
    Replaced(Attached<Querier, ParsedVersion<'a>>),
    /// The entry was occupied by a querier whose package version is not older than the provided querier.
    /// The occupied querier was kept, the provided querier was rejected.
    Rejected(Attached<Querier, ParsedVersion<'a>>),
}

impl<'a, Querier: ShouldReuse> MultiQueryDatabase<'a, Querier> {
    /// Add a querier of a `desc` file to the database unless the entry was already occupied by a querier whose
    /// package version is not older than the provided querier.
    fn insert_newer_with<GetName, GetVersion>(
        &mut self,
        repository: RepositoryName<'a>,
        mut querier: Querier,
        get_name: GetName,
        get_version: GetVersion,
    ) -> Result<InsertNewerReturn<'a, Querier>, InsertError<'a>>
    where
        GetName: FnOnce(&mut Querier) -> Option<Name<'a>>,
        GetVersion: FnOnce(&mut Querier) -> Option<Version<'a>>,
    {
        let name = get_name(&mut querier).ok_or(InsertError::NoName)?;
        let inserted_version = querier
            .pipe_mut(get_version)
            .ok_or(InsertError::NoVersion)?
            .parse()
            .map_err(InsertError::ParseVersion)?;

        let inserted = querier.into_attached(inserted_version);

        let multi_querier = self.internal.entry(&name).or_default();

        let Some(existing) = multi_querier.internal.get_mut(repository.as_str()) else {
            multi_querier.internal.insert(&repository, inserted);
            return Ok(InsertNewerReturn::Unoccupied);
        };

        let existing_version = existing.attachment();
        Ok(if existing_version < &inserted_version {
            InsertNewerReturn::Replaced(replace(existing, inserted))
        } else {
            InsertNewerReturn::Rejected(inserted)
        })
    }

    /// Add an [immutable querier](Query) of a `desc` file to the database unless the entry was already occupied by a querier whose
    /// package version is not older than the provided querier.
    pub fn insert_newer(
        &mut self,
        repository: RepositoryName<'a>,
        querier: Querier,
    ) -> Result<InsertNewerReturn<'a, Querier>, InsertError<'a>>
    where
        Querier: Query<'a>,
    {
        self.insert_newer_with(
            repository,
            querier,
            |querier| querier.name(),
            |querier| querier.version(),
        )
    }

    /// Add a [mutable querier](QueryMut) of a `desc` file to the database unless the entry was already occupied by a querier whose
    /// package version is not older than the provided querier.
    pub fn insert_newer_mut(
        &mut self,
        repository: RepositoryName<'a>,
        querier: Querier,
    ) -> Result<InsertNewerReturn<'a, Querier>, InsertError<'a>>
    where
        Querier: QueryMut<'a>,
    {
        self.insert_newer_with(repository, querier, Querier::name_mut, Querier::version_mut)
    }
}
