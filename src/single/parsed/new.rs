use super::SingleParsedDatabase;
use std::collections::HashMap;

impl<Querier> SingleParsedDatabase<'_, Querier> {
    /// Create an empty database.
    pub fn new() -> Self {
        SingleParsedDatabase {
            internal: HashMap::new(),
        }
    }
}

impl<Querier> Default for SingleParsedDatabase<'_, Querier> {
    fn default() -> Self {
        Self::new()
    }
}
