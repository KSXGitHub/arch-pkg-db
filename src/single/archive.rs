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

    /// Create the archive from a `Vec` of `String`s.
    pub fn from_vec(source: Vec<String>) -> Self {
        Archive { internal: source }
    }

    /// Convert the archive into a `Vec` of `String`s.
    pub fn into_vec(self) -> Vec<String> {
        self.internal
    }
}
