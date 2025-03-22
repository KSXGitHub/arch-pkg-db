use super::MultiQueryDatabase;
use crate::multi::RepositoryName;
use arch_pkg_text::desc::QueryMut;
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

/// Error type of [`QueryDatabase::insert`].
#[derive(Debug, Display, Error)]
#[display(bound())]
pub enum InsertError<Querier> {
    NoName(NoNameError<Querier>),
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
    ) -> Result<Option<Querier>, InsertError<Querier>> {
        let Some(name) = querier.name_mut() else {
            return Err(InsertError::NoName(NoNameError { querier }));
        };
        self.internal
            .entry(&name)
            .or_default()
            .internal
            .insert(&repository, querier)
            .pipe(Ok)
    }
}
