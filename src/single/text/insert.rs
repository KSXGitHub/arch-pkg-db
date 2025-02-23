use super::TextCollection;

impl TextCollection {
    /// Add data into the text collection.
    pub fn insert(&mut self, text: String) {
        self.internal.push(text);
    }
}
