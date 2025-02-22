use super::Archive;

impl Archive {
    /// Create the archive from a `Vec` of `String`s.
    pub fn from_vec(source: Vec<String>) -> Self {
        Archive { internal: source }
    }

    /// Convert the archive into a `Vec` of `String`s.
    pub fn into_vec(self) -> Vec<String> {
        self.internal
    }
}
