use super::QueryDatabase;
use std::collections::HashMap;

impl<Querier> QueryDatabase<'_, Querier> {
    /// Create an empty database.
    pub fn new() -> Self {
        QueryDatabase {
            internal: HashMap::new(),
        }
    }

    /// Create an empty database with at least the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        QueryDatabase {
            internal: HashMap::with_capacity(capacity),
        }
    }
}

impl<Querier> Default for QueryDatabase<'_, Querier> {
    fn default() -> Self {
        Self::new()
    }
}
