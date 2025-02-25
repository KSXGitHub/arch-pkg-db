use super::TextCollection;

impl TextCollection {
    /// Create the text collection from a `Vec` of `String`s.
    pub fn from_vec(source: Vec<Box<str>>) -> Self {
        TextCollection { internal: source }
    }

    /// Convert the text collection into a `Vec` of `String`s.
    pub fn into_vec(self) -> Vec<Box<str>> {
        self.internal
    }
}

impl From<TextCollection> for Vec<Box<str>> {
    fn from(value: TextCollection) -> Self {
        value.into_vec()
    }
}

impl From<Vec<Box<str>>> for TextCollection {
    fn from(value: Vec<Box<str>>) -> Self {
        TextCollection::from_vec(value)
    }
}
