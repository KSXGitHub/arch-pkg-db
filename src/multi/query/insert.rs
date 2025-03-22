use super::{MultiQueryDatabase, WithVersion};
use crate::{misc::IntoAttached, multi::RepositoryName};
use arch_pkg_text::{desc::QueryMut, value::ParseVersionError};
use derive_more::{Display, Error};
use pipe_trait::Pipe;

/// Error that occurs because a querier failed to provide a [name](QueryMut::name_mut).
#[derive(Debug, Display, Error)]
#[display(bound())]
#[display("Querier could not provide a name")]
pub struct NoNameError<Querier> {
    #[error(not(source))]
    pub querier: Querier,
}

/// Error that occurs because a querier failed to provide a [version](QueryMut::version_mut).
#[derive(Debug, Display, Error)]
#[display(bound())]
#[display("Querier could not provide a version")]
pub struct NoVersionError<Querier> {
    #[error(not(source))]
    pub querier: Querier,
}

/// Error type of [`QueryDatabase::insert`].
#[derive(Debug, Display, Error)]
#[display(bound())]
pub enum InsertError<'a, Querier> {
    NoName(NoNameError<Querier>),
    NoVersion(NoVersionError<Querier>),
    ParseVersion(#[error(not(source))] ParseVersionError<'a>),
}

impl<'a, Querier> MultiQueryDatabase<'a, Querier>
where
    Querier: QueryMut<'a>,
{
    /// Add a `desc` file to the database.
    ///
    /// If an older querier already occupied the same pair of [name] and [repository], it will be returned inside `Ok(Some(_))`.
    ///
    /// [name]: arch_pkg_text::value::Name
    /// [repository]: RepositoryName
    pub fn insert(
        &mut self,
        repository: RepositoryName<'a>,
        mut querier: Querier,
    ) -> Result<Option<WithVersion<'a, Querier>>, InsertError<Querier>> {
        let Some(name) = querier.name_mut() else {
            return Err(InsertError::NoName(NoNameError { querier }));
        };
        let Some(version) = querier.version_mut() else {
            return Err(InsertError::NoVersion(NoVersionError { querier }));
        };
        let version = version.parse().map_err(InsertError::ParseVersion)?;
        self.internal
            .entry(&name)
            .or_default()
            .internal
            .insert(&repository, querier.into_attached(version))
            .pipe(Ok)
    }
}
