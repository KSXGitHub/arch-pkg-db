use super::TextCollection;

impl TextCollection {
    /// Create an empty text collection.
    pub fn new() -> Self {
        TextCollection::default()
    }

    /// Create an empty text collection with specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        TextCollection {
            internal: Vec::with_capacity(capacity),
        }
    }
}
