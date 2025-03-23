use super::QueryDatabase;
use arch_pkg_text::{
    desc::{Query, QueryMut},
    misc::desc::ShouldReuse,
    value::{Name, ParseVersionError, Version},
};
use core::mem::replace;
use derive_more::{Display, Error};
use pipe_trait::Pipe;

/// Error type of [`QueryDatabase::insert`] and [`QueryDatabase::insert_mut`].
#[derive(Debug, Display, Clone, Copy, Error)]
pub enum InsertError {
    #[display("Querier could not provide a name")]
    NoName,
}

impl<'a, Querier: ShouldReuse> QueryDatabase<'a, Querier> {
    /// Add a querier of a `desc` file to the database.
    ///
    /// If an older querier already occupied the same [name](arch_pkg_text::value::Name), it will be returned inside `Ok(Some(_))`.
    fn insert_with<GetName>(
        &mut self,
        mut querier: Querier,
        get_name: GetName,
    ) -> Result<Option<Querier>, InsertError>
    where
        GetName: FnOnce(&mut Querier) -> Option<Name<'a>>,
    {
        let name = get_name(&mut querier).ok_or(InsertError::NoName)?;
        self.internal.insert(&name, querier).pipe(Ok)
    }

    /// Add an [immutable querier](Query) of a `desc` file to the database.
    ///
    /// If an older querier already occupied the same [name](arch_pkg_text::value::Name), it will be returned inside `Ok(Some(_))`.
    pub fn insert(&mut self, querier: Querier) -> Result<Option<Querier>, InsertError>
    where
        Querier: Query<'a>,
    {
        self.insert_with(querier, |querier| querier.name())
    }

    /// Add a [mutable querier](QueryMut) of a `desc` file to the database.
    ///
    /// If an older querier already occupied the same [name](arch_pkg_text::value::Name), it will be returned inside `Ok(Some(_))`.
    pub fn insert_mut(&mut self, querier: Querier) -> Result<Option<Querier>, InsertError>
    where
        Querier: QueryMut<'a>,
    {
        self.insert_with(querier, Querier::name_mut)
    }
}

/// Return type of [`QueryDatabase::insert_newer`] and [`QueryDatabase::insert_newer_mut`] upon success.
#[derive(Debug, Clone, Copy)]
pub enum InsertNewerReturn<Querier> {
    /// The entry was unoccupied, the querier was successfully inserted.
    Unoccupied,
    /// The entry was occupied by a querier whose package version is older than the provided querier.
    /// The provided querier thus replaced the old one.
    Replaced(Querier),
    /// The entry was occupied by a querier whose package version is not older than the provided querier.
    /// The occupied querier was kept, the provided querier was rejected.
    Rejected(Querier),
}

/// Error type of [`QueryDatabase::insert_newer`] and [`QueryDatabase::insert_newer_mut`].
#[derive(Debug, Display, Error)]
pub enum InsertNewerError<'a> {
    #[display("Querier could not provide a name")]
    NoName,
    #[display("Querier could not provide a version")]
    NoVersion,
    #[display("Version provided by the querier has invalid syntax: {_0}")]
    InvalidVersion(#[error(not(source))] ParseVersionError<'a>),
}

impl<'a, Querier: ShouldReuse> QueryDatabase<'a, Querier> {
    /// Add a querier of a `desc` file to the database unless the entry was already occupied by a querier whose
    /// package version is not older than the provided querier.
    fn insert_newer_with<GetName, GetVersion>(
        &mut self,
        mut querier: Querier,
        get_name: GetName,
        mut get_version: GetVersion,
    ) -> Result<InsertNewerReturn<Querier>, InsertNewerError<'a>>
    where
        GetName: FnOnce(&mut Querier) -> Option<Name<'a>>,
        GetVersion: FnMut(&mut Querier) -> Option<Version<'a>>,
    {
        let name = get_name(&mut querier).ok_or(InsertNewerError::NoName)?;
        let Some(existing) = self.internal.get_mut(name.as_str()) else {
            self.internal.insert(&name, querier);
            return Ok(InsertNewerReturn::Unoccupied);
        };

        let existing_version = existing
            .pipe_mut(&mut get_version)
            .ok_or(InsertNewerError::NoVersion)?
            .parse()
            .map_err(InsertNewerError::InvalidVersion)?;
        let inserted_version = querier
            .pipe_mut(&mut get_version)
            .ok_or(InsertNewerError::NoVersion)?
            .parse()
            .map_err(InsertNewerError::InvalidVersion)?;

        Ok(if existing_version < inserted_version {
            InsertNewerReturn::Replaced(replace(existing, querier))
        } else {
            InsertNewerReturn::Rejected(querier)
        })
    }

    /// Add an [immutable querier](Query) of a `desc` file to the database unless the entry was already occupied by a querier whose
    /// package version is not older than the provided querier.
    pub fn insert_newer(
        &mut self,
        querier: Querier,
    ) -> Result<InsertNewerReturn<Querier>, InsertNewerError<'a>>
    where
        Querier: Query<'a>,
    {
        self.insert_newer_with(
            querier,
            |querier| querier.name(),
            |querier| querier.version(),
        )
    }

    /// Add a [mutable querier](QueryMut) of a `desc` file to the database unless the entry was already occupied by a querier whose
    /// package version is not older than the provided querier.
    pub fn insert_newer_mut(
        &mut self,
        querier: Querier,
    ) -> Result<InsertNewerReturn<Querier>, InsertNewerError<'a>>
    where
        Querier: QueryMut<'a>,
    {
        self.insert_newer_with(querier, Querier::name_mut, Querier::version_mut)
    }
}
