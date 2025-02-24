use super::TextCollection;
use crate::{SingleParsedDatabase, single::parsed::AddError};
use arch_pkg_text::{ParsedDesc, parse::DescParseError};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(bound(ParseError: Display))]
pub enum TextCollectionParseError<Querier, ParseError> {
    Parse(ParseError),
    Add(AddError<Querier>),
}

type ParseEagerResult<'a> = Result<
    SingleParsedDatabase<'a, ParsedDesc<'a>>,
    TextCollectionParseError<ParsedDesc<'a>, DescParseError<'a>>,
>;

impl TextCollection {
    #[expect(clippy::result_large_err)] // until `parse_*_with_issue`.
    pub fn parse_eager(&self) -> ParseEagerResult<'_> {
        let mut db = SingleParsedDatabase::new();

        for text in &self.internal {
            let querier = ParsedDesc::parse(text).map_err(TextCollectionParseError::Parse)?;
            db.add(querier).map_err(TextCollectionParseError::Add)?;
        }

        Ok(db)
    }
}
