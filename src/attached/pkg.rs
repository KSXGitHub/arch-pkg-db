use super::Attached;
use arch_pkg_text::desc::{ParsedField, Query, QueryMut};

impl<'a, Querier: Query<'a>, Attachment> Query<'a> for Attached<Querier, Attachment> {
    fn query_raw_text(&self, field: ParsedField) -> Option<&'a str> {
        Querier::query_raw_text(self, field)
    }
}

impl<'a, Querier: QueryMut<'a>, Attachment> QueryMut<'a> for Attached<Querier, Attachment> {
    fn query_raw_text_mut(&mut self, field: ParsedField) -> Option<&'a str> {
        Querier::query_raw_text_mut(self, field)
    }
}
