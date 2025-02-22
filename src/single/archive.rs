/// Collections of all `desc` texts from which queriers may access data.
#[derive(Debug, Default, Clone)]
pub struct Archive {
    internal: Vec<String>,
}

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

    /// Add data into the archive.
    pub fn insert(&mut self, text: String) {
        self.internal.push(text);
    }

    /// Shrink the capacity of the internal data.
    pub fn shrink_to_fit(&mut self) {
        self.internal.shrink_to_fit()
    }

    /// Extract the internal data from the archive.
    pub fn into_internal(self) -> Vec<String> {
        self.internal
    }
}
