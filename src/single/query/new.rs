use super::QueryDatabase;
use std::collections::HashMap;

impl<Querier> QueryDatabase<'_, Querier> {
    /// Create an empty database.
    pub fn new() -> Self {
        QueryDatabase {
            internal: HashMap::new(),
        }
    }
}

impl<Querier> Default for QueryDatabase<'_, Querier> {
    fn default() -> Self {
        Self::new()
    }
}
