use super::{Text, TextCollection};

impl TextCollection {
    /// Add data into the text collection.
    pub fn insert(&mut self, text: Text) {
        self.internal.push(text);
    }

    /// Add data into the text collection.
    pub fn add_item(mut self, text: Text) -> Self {
        self.insert(text);
        self
    }
}
