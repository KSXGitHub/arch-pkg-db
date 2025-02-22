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

impl From<Archive> for Vec<String> {
    fn from(value: Archive) -> Self {
        value.into_vec()
    }
}

impl From<Vec<String>> for Archive {
    fn from(value: Vec<String>) -> Self {
        Archive::from_vec(value)
    }
}
