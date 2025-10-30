use super::MultiTextCollection;
use crate::{Text, desc::value::RepositoryName};

impl<'a> Extend<(RepositoryName<'a>, Text)> for MultiTextCollection<'a> {
    fn extend<Iter: IntoIterator<Item = (RepositoryName<'a>, Text)>>(&mut self, iter: Iter) {
        let iter = iter.into_iter();
        let (cap, _) = iter.size_hint();
        self.internal.reserve(cap);
        for (repository, text) in iter {
            self.insert(repository, text);
        }
    }
}
