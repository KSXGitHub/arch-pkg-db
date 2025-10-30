use super::MultiTextCollection;
use crate::{MultiQueryDatabase, Text, multi::query::InsertError, value::RepositoryName};
use arch_pkg_text::desc::{Query, QueryMut, misc::ShouldReuse};
use core::error::Error;
use derive_more::Display;
use pipe_trait::Pipe;
use rayon::prelude::*;

/// Error type when trying to create a [`MultiQueryDatabase`] from a [`MultiTextCollection`].
#[derive(Debug, Display, Clone)]
#[display(bound(ParseError: Display))]
pub enum MultiTextCollectionParseError<'a, ParseError> {
    Parse(ParseError),
    Insert(InsertError<'a>),
}

// We implement Error manually because derive_more::Error was unable to handle it.
impl<'a, ParseError: Error> Error for MultiTextCollectionParseError<'a, ParseError> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MultiTextCollectionParseError::Parse(error) => error.source(),
            MultiTextCollectionParseError::Insert(error) => error.source(),
        }
    }
}

/// Return type of [`MultiTextCollection::parse`] and [`MultiTextCollection::parse_mut`].
type ParseResult<'a, Querier> = Result<
    MultiQueryDatabase<'a, Querier>,
    MultiTextCollectionParseError<'a, <&'a str as TryInto<Querier>>::Error>,
>;

impl<'a> MultiTextCollection<'a> {
    /// Parse a database of queriers.
    fn parse_with<Querier, Insert, InsertSuccess>(
        &'a self,
        mut insert: Insert,
    ) -> ParseResult<'a, Querier>
    where
        &'a str: TryInto<Querier>,
        Insert: FnMut(
            &mut MultiQueryDatabase<'a, Querier>,
            RepositoryName<'a>,
            Querier,
        ) -> Result<InsertSuccess, InsertError<'a>>,
    {
        let iter = self.iter();
        let (lower_cap, _) = iter.size_hint();
        let mut db = MultiQueryDatabase::with_capacity(lower_cap);

        for (repository, text) in iter {
            let querier = text
                .as_str()
                .try_into()
                .map_err(MultiTextCollectionParseError::Parse)?;
            insert(&mut db, repository, querier).map_err(MultiTextCollectionParseError::Insert)?;
        }

        Ok(db)
    }

    /// Parse a database of [immutable queriers](Query).
    pub fn parse<Querier>(&'a self) -> ParseResult<'a, Querier>
    where
        &'a str: TryInto<Querier>,
        Querier: Query<'a> + ShouldReuse,
    {
        self.parse_with(MultiQueryDatabase::insert)
    }

    /// Parse a database of [mutable queriers](QueryMut).
    pub fn parse_mut<Querier>(&'a self) -> ParseResult<'a, Querier>
    where
        &'a str: TryInto<Querier>,
        Querier: QueryMut<'a> + ShouldReuse,
    {
        self.parse_with(MultiQueryDatabase::insert_mut)
    }

    /// Parse a database of queriers in parallel.
    fn par_parse_with<Querier, QueriersIntoDb>(
        &'a self,
        queriers_into_db: QueriersIntoDb,
    ) -> ParseResult<'a, Querier>
    where
        &'a str: TryInto<Querier, Error: Send>,
        Querier: Send,
        QueriersIntoDb: FnOnce(
            Vec<(RepositoryName<'a>, Querier)>,
        ) -> Result<MultiQueryDatabase<'a, Querier>, InsertError>,
    {
        self.internal
            .par_iter()
            .flat_map(|(repository, collection)| {
                collection
                    .iter()
                    .par_bridge() // TODO: maybe implement ParallelIterator?
                    .map(Text::as_str)
                    .map(TryInto::<Querier>::try_into)
                    .map(move |querier| querier.map(|querier| (*repository, querier)))
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(MultiTextCollectionParseError::Parse)?
            .pipe(queriers_into_db)
            .map_err(MultiTextCollectionParseError::Insert)
    }

    /// Parse a database of [immutable queriers](Query) in parallel.
    pub fn par_parse<Querier>(&'a self) -> ParseResult<'a, Querier>
    where
        &'a str: TryInto<Querier, Error: Send>,
        Querier: Query<'a> + ShouldReuse + Send,
    {
        self.par_parse_with(MultiQueryDatabase::from_queriers)
    }

    /// Parse a database of [mutable queriers](QueryMut) in parallel.
    pub fn par_parse_mut<Querier>(&'a self) -> ParseResult<'a, Querier>
    where
        &'a str: TryInto<Querier, Error: Send>,
        Querier: QueryMut<'a> + ShouldReuse + Send,
    {
        self.par_parse_with(MultiQueryDatabase::from_queriers_mut)
    }
}
