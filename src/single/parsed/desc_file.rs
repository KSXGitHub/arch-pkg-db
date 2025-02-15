use arch_pkg_text::desc::{ParsedField, Query, QueryMut};
use std::path::Path;

/// Representation of a `desc` file inside [`SingleParsedDatabase`](super::SingleParsedDatabase).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DescFile<'a, Querier> {
    pub path: &'a Path,
    pub querier: Querier,
}

impl<'a, Querier> DescFile<'a, Querier> {
    /// Get a view of the `desc` file with an immutable querier.
    pub fn as_deref(&self) -> DescFile<'a, &'_ Querier> {
        let DescFile { path, querier } = self;
        let path = *path;
        DescFile { path, querier }
    }

    /// Get a view of the `desc` file with a mutable querier.
    pub fn as_deref_mut(&mut self) -> DescFile<'a, &'_ mut Querier> {
        let DescFile { path, querier } = self;
        let path = *path;
        DescFile { path, querier }
    }
}

impl<'a, Querier: Query<'a>> Query<'a> for DescFile<'a, &Querier> {
    fn query_raw_text(&self, field: ParsedField) -> Option<&'a str> {
        self.querier.query_raw_text(field)
    }
}

impl<'a, Querier: Query<'a>> QueryMut<'a> for DescFile<'a, &Querier> {
    fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
        self.query_raw_text(field)
    }
}

impl<'a, Querier: QueryMut<'a>> QueryMut<'a> for DescFile<'a, &mut Querier> {
    fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
        self.querier.query_raw_text_mut(field)
    }
}
