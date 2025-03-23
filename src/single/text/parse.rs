use super::TextCollection;
use crate::{QueryDatabase, single::query::InsertError};
use arch_pkg_text::desc::{Query, QueryMut, misc::ShouldReuse};
use derive_more::{Display, Error};

/// Error type when trying to create a [`QueryDatabase`] from a [`TextCollection`].
///
/// [`QueryDatabase`]: crate::QueryDatabase
#[derive(Debug, Display, Clone, Copy, Error)]
#[display(bound(ParseError: Display))]
pub enum TextCollectionParseError<ParseError> {
    Parse(ParseError),
    Insert(InsertError),
}

/// Return type of [`TextCollection::parse`] and [`TextCollection::parse_mut`].
type ParseResult<'a, Querier> = Result<
    QueryDatabase<'a, Querier>,
    TextCollectionParseError<<&'a str as TryInto<Querier>>::Error>,
>;

impl TextCollection {
    /// Parse a database of queriers.
    fn parse_with<'a, Querier, Insert>(&'a self, mut insert: Insert) -> ParseResult<'a, Querier>
    where
        &'a str: TryInto<Querier>,
        Insert: FnMut(&mut QueryDatabase<'a, Querier>, Querier) -> Result<(), InsertError>,
    {
        let mut db = QueryDatabase::new();

        for text in &self.internal {
            let querier = text
                .as_str()
                .try_into()
                .map_err(TextCollectionParseError::Parse)?;
            insert(&mut db, querier).map_err(TextCollectionParseError::Insert)?;
        }

        Ok(db)
    }

    /// Parse a database of [immutable queriers](Query).
    pub fn parse<'a, Querier>(&'a self) -> ParseResult<'a, Querier>
    where
        &'a str: TryInto<Querier>,
        Querier: Query<'a> + ShouldReuse,
    {
        self.parse_with(|db, querier| db.insert(querier).map(drop))
    }

    /// Parse a database of [mutable queriers](QueryMut).
    pub fn parse_mut<'a, Querier>(&'a self) -> ParseResult<'a, Querier>
    where
        &'a str: TryInto<Querier>,
        Querier: QueryMut<'a> + ShouldReuse,
    {
        self.parse_with(|db, querier| db.insert_mut(querier).map(drop))
    }
}
