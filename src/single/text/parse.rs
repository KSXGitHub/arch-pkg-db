use super::{Text, TextCollection};
use crate::{QueryDatabase, single::query::InsertError};
use arch_pkg_text::desc::{Query, QueryMut, misc::ShouldReuse};
use derive_more::{Display, Error};
use pipe_trait::Pipe;
use rayon::prelude::*;

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
        let mut db = QueryDatabase::with_capacity(self.internal.len());

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

    /// Parse a database of queriers in parallel.
    fn par_parse_with<'a, Querier, QueriersIntoDb>(
        &'a self,
        queriers_into_db: QueriersIntoDb,
    ) -> ParseResult<'a, Querier>
    where
        &'a str: TryInto<Querier, Error: Send>,
        Querier: Send,
        QueriersIntoDb: FnOnce(Vec<Querier>) -> Result<QueryDatabase<'a, Querier>, InsertError>,
    {
        self.internal
            .par_iter()
            .map(Text::as_str)
            .map(TryInto::<Querier>::try_into)
            .collect::<Result<Vec<_>, _>>()
            .map_err(TextCollectionParseError::Parse)?
            .pipe(queriers_into_db)
            .map_err(TextCollectionParseError::Insert)
    }

    /// Parse a database of [immutable queriers](Query) in parallel.
    pub fn par_parse<'a, Querier>(&'a self) -> ParseResult<'a, Querier>
    where
        &'a str: TryInto<Querier, Error: Send>,
        Querier: Query<'a> + ShouldReuse + Send,
    {
        self.par_parse_with(QueryDatabase::from_queriers)
    }

    /// Parse a database of [mutable queriers](QueryMut) in parallel.
    pub fn par_parse_mut<'a, Querier>(&'a self) -> ParseResult<'a, Querier>
    where
        &'a str: TryInto<Querier, Error: Send>,
        Querier: QueryMut<'a> + ShouldReuse + Send,
    {
        self.par_parse_with(QueryDatabase::from_queriers_mut)
    }
}
