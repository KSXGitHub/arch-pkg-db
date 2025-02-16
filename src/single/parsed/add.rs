use super::SingleParsedDatabase;
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

/// Error type of [`SingleParsedDatabase::add`].
#[derive(Debug, Display, Error)]
#[display(bound())]
pub enum AddError<Querier> {
    NoName(NoNameError<Querier>),
}

impl<'a, Querier> SingleParsedDatabase<'a, Querier>
where
    Querier: QueryMut<'a>,
{
    /// Add a `desc` file to the database.
    ///
    /// If an older `Querier` already occupied the same [name](arch_pkg_text::value::Name), it will be returned inside `Ok(Some(_))`.
    pub fn add(&mut self, mut querier: Querier) -> Result<Option<Querier>, AddError<Querier>> {
        if let Some(name) = querier.name_mut() {
            self.internal.insert(name, querier).pipe(Ok)
        } else {
            Err(AddError::NoName(NoNameError { querier }))
        }
    }
}
