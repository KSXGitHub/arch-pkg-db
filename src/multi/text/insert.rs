use super::MultiTextCollection;
use crate::{misc::Text, multi::RepositoryName};

impl<'a> MultiTextCollection<'a> {
    /// Add data to the text collection.
    pub fn insert(&mut self, repository: RepositoryName<'a>, text: Text) {
        self.internal.push((repository, text));
    }

    /// Add data to the text collection.
    pub fn add_item(mut self, repository: RepositoryName<'a>, text: Text) -> Self {
        self.insert(repository, text);
        self
    }
}
