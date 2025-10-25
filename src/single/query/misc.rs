use super::QueryDatabase;

impl<'a, Querier> QueryDatabase<'a, Querier> {
    /// The number of queriers within the database.
    pub fn len(&self) -> usize {
        self.internal.len()
    }

    /// Whether the database is empty.
    pub fn is_empty(&self) -> bool {
        self.internal.is_empty()
    }
}
