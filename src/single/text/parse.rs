use super::TextCollection;
use crate::{QueryDatabase, single::query::AddError};
use arch_pkg_text::desc::{
    QueryMut,
    misc::{ReuseAdvice, True},
};
use derive_more::{Display, Error};

/// Error type when trying to create a [`QueryDatabase`] from a [`TextCollection`].
///
/// [`QueryDatabase`]: crate::QueryDatabase
#[derive(Debug, Display, Error)]
#[display(bound(ParseError: Display))]
pub enum TextCollectionParseError<Querier, ParseError> {
    Parse(ParseError),
    Add(AddError<Querier>),
}

/// Return type of [`TextCollection::parse`].
type ParseResult<'a, Querier> = Result<
    QueryDatabase<'a, Querier>,
    TextCollectionParseError<Querier, <&'a str as TryInto<Querier>>::Error>,
>;

impl TextCollection {
    /// Parse a database of queriers.
    pub fn parse<'a, Querier>(&'a self) -> ParseResult<'a, Querier>
    where
        &'a str: TryInto<Querier>,
        Querier: QueryMut<'a> + ReuseAdvice<ShouldReuse = True>,
    {
        let mut db = QueryDatabase::new();

        for text in &self.internal {
            let querier = text
                .as_str()
                .try_into()
                .map_err(TextCollectionParseError::Parse)?;
            db.add(querier).map_err(TextCollectionParseError::Add)?;
        }

        Ok(db)
    }
}
