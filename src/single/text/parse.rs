use super::TextCollection;
use crate::{EagerQueryDatabase, single::query::AddError};
use arch_pkg_text::{ParsedDesc, parse::DescParseError};
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

/// Return type of [`TextCollection::parse_eager`].
type ParseEagerResult<'a> =
    Result<EagerQueryDatabase<'a>, TextCollectionParseError<ParsedDesc<'a>, DescParseError<'a>>>;

impl TextCollection {
    /// Parse a database of eager queriers.
    #[expect(clippy::result_large_err)] // until `parse_*_with_issue`.
    pub fn parse_eager(&self) -> ParseEagerResult<'_> {
        let mut db = EagerQueryDatabase::new();

        for text in &self.internal {
            let querier = ParsedDesc::parse(text).map_err(TextCollectionParseError::Parse)?;
            db.add(querier).map_err(TextCollectionParseError::Add)?;
        }

        Ok(db)
    }
}
