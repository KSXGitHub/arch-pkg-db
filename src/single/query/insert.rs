use super::QueryDatabase;
use arch_pkg_text::{
    desc::{Query, QueryMut},
    misc::desc::ShouldReuse,
    value::Name,
};
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

impl<'a, Querier: ShouldReuse> QueryDatabase<'a, Querier> {
    /// Add a querier of a `desc` file to the database.
    ///
    /// If an older querier already occupied the same [name](arch_pkg_text::value::Name), it will be returned inside `Ok(Some(_))`.
    fn insert_with<GetName>(
        &mut self,
        mut querier: Querier,
        get_name: GetName,
    ) -> Result<Option<Querier>, InsertError<Querier>>
    where
        GetName: FnOnce(&mut Querier) -> Option<Name<'a>>,
    {
        if let Some(name) = get_name(&mut querier) {
            self.internal.insert(name.as_str(), querier).pipe(Ok)
        } else {
            Err(InsertError::NoName(NoNameError { querier }))
        }
    }

    /// Add an [immutable querier](Query) of a `desc` file to the database.
    ///
    /// If an older querier already occupied the same [name](arch_pkg_text::value::Name), it will be returned inside `Ok(Some(_))`.
    pub fn insert(&mut self, querier: Querier) -> Result<Option<Querier>, InsertError<Querier>>
    where
        Querier: Query<'a>,
    {
        self.insert_with(querier, |querier| querier.name())
    }

    /// Add a [mutable querier](QueryMut) of a `desc` file to the database.
    ///
    /// If an older querier already occupied the same [name](arch_pkg_text::value::Name), it will be returned inside `Ok(Some(_))`.
    pub fn insert_mut(&mut self, querier: Querier) -> Result<Option<Querier>, InsertError<Querier>>
    where
        Querier: QueryMut<'a>,
    {
        self.insert_with(querier, Querier::name_mut)
    }
}
