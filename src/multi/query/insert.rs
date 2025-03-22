use super::{MultiQueryDatabase, WithVersion};
use crate::{misc::IntoAttached, multi::RepositoryName};
use arch_pkg_text::{
    desc::{Query, QueryMut, misc::ShouldReuse},
    value::ParseVersionError,
};
use derive_more::{Display, Error};
use pipe_trait::Pipe;

/// Error type of [`QueryDatabase::insert`].
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
        let name = querier.name().ok_or(InsertError::NoName)?;
        let version = querier
            .version()
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

    /// Add a [mutable querier](QueryMut) of a `desc` file to the database.
    ///
    /// If an older querier already occupied the same pair of [name] and [repository], it will be returned inside `Ok(Some(_))`.
    ///
    /// [name]: arch_pkg_text::value::Name
    /// [repository]: RepositoryName
    pub fn insert_mut(
        &mut self,
        repository: RepositoryName<'a>,
        mut querier: Querier,
    ) -> Result<Option<WithVersion<'a, Querier>>, InsertError<'a>>
    where
        Querier: QueryMut<'a>,
    {
        let name = querier.name_mut().ok_or(InsertError::NoName)?;
        let version = querier
            .version_mut()
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
}
