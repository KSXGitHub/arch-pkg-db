use super::MultiTextCollection;

impl MultiTextCollection<'_> {
    /// Create an empty text collection.
    pub fn new() -> Self {
        MultiTextCollection::default()
    }

    /// Create an empty text collection with specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        MultiTextCollection {
            internal: Vec::with_capacity(capacity),
        }
    }
}
