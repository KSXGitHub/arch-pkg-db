use super::MultiTextCollection;
use crate::{Text, TextCollection, desc::value::RepositoryName};

impl<'a> FromIterator<(RepositoryName<'a>, TextCollection)> for MultiTextCollection<'a> {
    fn from_iter<Iter: IntoIterator<Item = (RepositoryName<'a>, TextCollection)>>(
        iter: Iter,
    ) -> Self {
        MultiTextCollection {
            internal: Vec::from_iter(iter),
        }
    }
}

impl<'a> FromIterator<(RepositoryName<'a>, Text)> for MultiTextCollection<'a> {
    fn from_iter<Iter: IntoIterator<Item = (RepositoryName<'a>, Text)>>(iter: Iter) -> Self {
        let mut col = MultiTextCollection::new();
        col.extend(iter);
        col
    }
}
