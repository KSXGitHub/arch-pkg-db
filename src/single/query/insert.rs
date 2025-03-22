use super::QueryDatabase;
use arch_pkg_text::{
    desc::{Query, QueryMut},
    misc::desc::ShouldReuse,
    value::Name,
};
use derive_more::{Display, Error};
use pipe_trait::Pipe;

/// Error type of [`QueryDatabase::insert`].
#[derive(Debug, Display, Error)]
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
        self.internal.insert(name.as_str(), querier).pipe(Ok)
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
