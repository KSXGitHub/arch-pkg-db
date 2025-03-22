use super::{MultiQuerier, MultiQueryDatabase};
use std::collections::HashMap;

impl<Querier> MultiQuerier<'_, Querier> {
    /// Create an empty queriers.
    pub fn new() -> Self {
        MultiQuerier {
            internal: HashMap::new(),
        }
    }
}

impl<Querier> Default for MultiQuerier<'_, Querier> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Querier> MultiQueryDatabase<'_, Querier> {
    /// Create an empty database.
    pub fn new() -> Self {
        MultiQueryDatabase {
            internal: HashMap::new(),
        }
    }
}

impl<Querier> Default for MultiQueryDatabase<'_, Querier> {
    fn default() -> Self {
        Self::new()
    }
}
