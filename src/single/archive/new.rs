use super::Archive;

impl Archive {
    /// Create an empty archive.
    pub fn new() -> Self {
        Archive::default()
    }

    /// Create an empty archive with specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Archive {
            internal: Vec::with_capacity(capacity),
        }
    }
}
