use super::MultiTextCollection;
use crate::{TextCollection, desc::value::RepositoryName};

impl<'a> MultiTextCollection<'a> {
    /// Add data into the multi-collection.
    pub fn insert(&mut self, repository: RepositoryName<'a>, collection: TextCollection) {
        self.internal.push((repository, collection));
    }

    /// Add data into the multi-collection.
    pub fn add_collection(
        mut self,
        repository: RepositoryName<'a>,
        collection: TextCollection,
    ) -> Self {
        self.insert(repository, collection);
        self
    }
}
