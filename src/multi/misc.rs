use super::{MultiQuerier, MultiQueryDatabase};

impl<Querier> MultiQuerier<'_, Querier> {
    /// The number of repositories that have a package of this name.
    pub fn len(&self) -> usize {
        self.internal.len()
    }

    /// Whether there are repositories that have a package of this name.
    pub fn is_empty(&self) -> bool {
        self.internal.is_empty()
    }
}

impl<Querier> MultiQueryDatabase<'_, Querier> {
    /// The number of package names within the database.
    pub fn len(&self) -> usize {
        self.internal.len()
    }

    /// Whether the database is empty.
    pub fn is_empty(&self) -> bool {
        self.internal.is_empty()
    }
}
