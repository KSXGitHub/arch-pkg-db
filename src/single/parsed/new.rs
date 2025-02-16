use super::SingleParsedDatabase;
use arch_pkg_text::indexmap::IndexMap;

impl<Querier> SingleParsedDatabase<'_, Querier> {
    /// Create an empty database.
    pub fn new() -> Self {
        SingleParsedDatabase {
            internal: IndexMap::new(),
        }
    }
}

impl<Querier> Default for SingleParsedDatabase<'_, Querier> {
    fn default() -> Self {
        Self::new()
    }
}
