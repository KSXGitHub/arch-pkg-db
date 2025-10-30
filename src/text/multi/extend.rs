use super::MultiTextCollection;
use crate::{Text, TextCollection, value::RepositoryName};
use itertools::Itertools;

impl<'a> Extend<(RepositoryName<'a>, TextCollection)> for MultiTextCollection<'a> {
    fn extend<Iter: IntoIterator<Item = (RepositoryName<'a>, TextCollection)>>(
        &mut self,
        iter: Iter,
    ) {
        let iter = iter.into_iter();
        let (cap, _) = iter.size_hint();
        self.internal.reserve(cap);
        for (repository, collection) in iter {
            self.insert(repository, collection);
        }
    }
}

impl<'a> Extend<(RepositoryName<'a>, Text)> for MultiTextCollection<'a> {
    fn extend<Iter: IntoIterator<Item = (RepositoryName<'a>, Text)>>(&mut self, iter: Iter) {
        let chunks = iter.into_iter().chunk_by(|&(repository, _)| repository);
        let iter = chunks.into_iter().map(|(repository, group)| {
            (
                repository,
                group
                    .inspect(|(expected, _)| debug_assert_eq!(&repository, expected))
                    .map(|(_, text)| text)
                    .collect::<TextCollection>(),
            )
        });
        self.extend(iter);
    }
}
