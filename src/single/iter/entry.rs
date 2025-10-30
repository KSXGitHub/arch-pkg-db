use arch_pkg_text::{
    desc::{FieldName, ParsedField, Query, QueryMut},
    value::Name,
};
use pipe_trait::Pipe;

/// An entry of a querier and its name.
///
/// This type would implement [`Query`] and [`QueryMut`] with pre-computed name
/// if the querier type already implemented them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry<'a, Querier> {
    name: Name<'a>,
    querier: Querier,
}

impl<'a, Querier> Entry<'a, Querier> {
    /// Create an entry without checking whether the names match.
    pub(super) fn new_unchecked(name: &'a str, querier: Querier) -> Self {
        Entry {
            name: Name(name),
            querier,
        }
    }

    /// Dissolve the entry into an owned tuple name and querier.
    pub fn into_tuple(self) -> (Name<'a>, Querier) {
        let Entry { name, querier } = self;
        (name, querier)
    }

    /// Get the name of the querier.
    pub fn name(&self) -> Name<'a> {
        self.name
    }

    /// Get an immutable reference to the querier inside.
    pub fn querier(&self) -> &Querier {
        &self.querier
    }

    /// Get a mutable reference to the querier inside.
    pub fn querier_mut(&mut self) -> &mut Querier {
        &mut self.querier
    }

    /// Get the owned querier inside.
    pub fn into_querier(self) -> Querier {
        self.querier
    }
}

impl<'a, Querier> Query<'a> for Entry<'a, Querier>
where
    Querier: Query<'a>,
{
    fn query_raw_text(&self, field: ParsedField) -> Option<&'a str> {
        match field.name() {
            FieldName::Name => self.name().as_str().pipe(Some),
            _ => self.querier.query_raw_text(field),
        }
    }

    fn name(&self) -> Option<Name<'a>> {
        self.name().pipe(Some)
    }
}

impl<'a, Querier> QueryMut<'a> for Entry<'a, Querier>
where
    Querier: QueryMut<'a>,
{
    fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
        match field.name() {
            FieldName::Name => self.name().as_str().pipe(Some),
            _ => self.querier.query_raw_text_mut(field),
        }
    }

    fn name_mut(&mut self) -> Option<Name<'a>> {
        self.name().pipe(Some)
    }
}
