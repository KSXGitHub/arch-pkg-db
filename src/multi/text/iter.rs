use super::MultiTextCollection;
use crate::{misc::Text, multi::RepositoryName};
use std::{iter::FusedIterator, slice, vec};

/// [Iterator] over immutable references to all items inside a [`MultiTextCollection`].
#[derive(Debug, Clone)]
pub struct MultiTextIter<'r, 'name> {
    internal: slice::Iter<'r, (RepositoryName<'name>, Text)>,
}

impl<'r, 'name> Iterator for MultiTextIter<'r, 'name> {
    type Item = &'r (RepositoryName<'name>, Text);

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.internal.size_hint()
    }

    fn count(self) -> usize {
        self.internal.count()
    }
}

impl DoubleEndedIterator for MultiTextIter<'_, '_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.internal.next_back()
    }
}

impl ExactSizeIterator for MultiTextIter<'_, '_> {
    fn len(&self) -> usize {
        self.internal.len()
    }
}

impl FusedIterator for MultiTextIter<'_, '_> {}

impl<'a> MultiTextCollection<'a> {
    /// Iterate over immutable references to the items inside.
    pub fn iter(&self) -> MultiTextIter<'_, 'a> {
        MultiTextIter {
            internal: self.internal.iter(),
        }
    }
}

impl<'r, 'name> IntoIterator for &'r MultiTextCollection<'name> {
    type Item = &'r (RepositoryName<'name>, Text);
    type IntoIter = MultiTextIter<'r, 'name>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// [Iterator] over mutable references to all items inside a [`MultiTextCollection`].
#[derive(Debug)]
pub struct MultiTextIterMut<'r, 'name> {
    internal: slice::IterMut<'r, (RepositoryName<'name>, Text)>,
}

impl<'r, 'name> Iterator for MultiTextIterMut<'r, 'name> {
    type Item = &'r mut (RepositoryName<'name>, Text);

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.internal.size_hint()
    }

    fn count(self) -> usize {
        self.internal.count()
    }
}

impl DoubleEndedIterator for MultiTextIterMut<'_, '_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.internal.next_back()
    }
}

impl ExactSizeIterator for MultiTextIterMut<'_, '_> {
    fn len(&self) -> usize {
        self.internal.len()
    }
}

impl FusedIterator for MultiTextIterMut<'_, '_> {}

impl<'a> MultiTextCollection<'a> {
    /// Iterate over mutable references to the items inside.
    pub fn iter_mut(&mut self) -> MultiTextIterMut<'_, 'a> {
        MultiTextIterMut {
            internal: self.internal.iter_mut(),
        }
    }
}

impl<'r, 'name> IntoIterator for &'r mut MultiTextCollection<'name> {
    type Item = &'r mut (RepositoryName<'name>, Text);
    type IntoIter = MultiTextIterMut<'r, 'name>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// [Iterator] over owned items inside a [`MultiTextCollection`].
#[derive(Debug, Clone)]
pub struct MultiTextIntoIter<'a> {
    internal: vec::IntoIter<(RepositoryName<'a>, Text)>,
}

impl<'a> Iterator for MultiTextIntoIter<'a> {
    type Item = (RepositoryName<'a>, Text);

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.internal.size_hint()
    }

    fn count(self) -> usize {
        self.internal.count()
    }
}

impl DoubleEndedIterator for MultiTextIntoIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.internal.next_back()
    }
}

impl ExactSizeIterator for MultiTextIntoIter<'_> {
    fn len(&self) -> usize {
        self.internal.len()
    }
}

impl FusedIterator for MultiTextIntoIter<'_> {}

impl<'a> IntoIterator for MultiTextCollection<'a> {
    type Item = (RepositoryName<'a>, Text);
    type IntoIter = MultiTextIntoIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        MultiTextIntoIter {
            internal: self.internal.into_iter(),
        }
    }
}
