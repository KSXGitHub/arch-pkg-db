use super::Archive;

impl Archive {
    /// Add data into the archive.
    pub fn insert(&mut self, text: String) {
        self.internal.push(text);
    }
}
