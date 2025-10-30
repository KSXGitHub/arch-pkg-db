use super::MultiTextCollection;
use crate::{
    Text, TextCollection,
    desc::value::RepositoryName,
    text::iter::{TextIntoIter, TextIter, TextIterMut},
};
use core::{iter::FusedIterator, slice};
use std::vec;

/// [Iterator] over immutable references to all items inside a [`MultiTextCollection`], each paired with
/// their corresponding [`RepositoryName`].
#[derive(Debug, Clone)]
pub struct MultiTextIter<'a> {
    current: Option<(RepositoryName<'a>, TextIter<'a>)>,
    remaining: slice::Iter<'a, (RepositoryName<'a>, TextCollection)>,
}

impl<'a> MultiTextIter<'a> {
    /// Construct an (effectively) empty iterator.
    ///
    /// This iterator requires at least one call to [`MultiTextIter::next_stage`] to become useful (i.e. non-empty).
    fn blank(collections: &'a Vec<(RepositoryName<'a>, TextCollection)>) -> Self {
        MultiTextIter {
            current: None,
            remaining: collections.iter(),
        }
    }

    /// Extract an item from [`MultiTextIter::remaining`] into [`MultiTextIter::current`] after it has been exhausted.
    fn next_stage(&mut self) {
        debug_assert!(
            self.current.is_none(),
            "next_stage must only be called after current has been exhausted",
        );
        self.current = self
            .remaining
            .next()
            .map(|(repository, collection)| (*repository, collection.iter()));
    }
}

impl<'a> Iterator for MultiTextIter<'a> {
    type Item = (RepositoryName<'a>, &'a Text);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (repository, text_iter) = self.current.as_mut()?;

            if let Some(text) = text_iter.next() {
                return Some((*repository, text));
            }

            self.next_stage();
        }
    }
}

impl FusedIterator for MultiTextIter<'_> {}

impl<'a> MultiTextCollection<'a> {
    /// Iterate over immutable references to all items inside a [`MultiTextCollection`], each paired with
    /// their corresponding [`RepositoryName`].
    pub fn iter(&'a self) -> MultiTextIter<'a> {
        let mut iter = MultiTextIter::blank(&self.internal);
        iter.next_stage();
        iter
    }
}

/// [Iterator] over mutable references to all items inside a [`MultiTextCollection`], each paired with
/// their corresponding [`RepositoryName`].
#[derive(Debug)]
pub struct MultiTextIterMut<'a> {
    current: Option<(RepositoryName<'a>, TextIterMut<'a>)>,
    remaining: slice::IterMut<'a, (RepositoryName<'a>, TextCollection)>,
}

impl<'a> MultiTextIterMut<'a> {
    /// Construct an (effectively) empty iterator.
    ///
    /// This iterator requires at least one call to [`MultiTextIterMut::next_stage`] to become useful (i.e. non-empty).
    fn blank(collections: &'a mut Vec<(RepositoryName<'a>, TextCollection)>) -> Self {
        MultiTextIterMut {
            current: None,
            remaining: collections.iter_mut(),
        }
    }

    /// Extract an item from [`MultiTextIterMut::remaining`] into [`MultiTextIterMut::current`] after it has been exhausted.
    fn next_stage(&mut self) {
        debug_assert!(
            self.current.is_none(),
            "next_stage must only be called after current has been exhausted",
        );
        self.current = self
            .remaining
            .next()
            .map(|(repository, collection)| (*repository, collection.iter_mut()));
    }
}

impl<'a> Iterator for MultiTextIterMut<'a> {
    type Item = (RepositoryName<'a>, &'a mut Text);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (repository, text_iter) = self.current.as_mut()?;

            if let Some(text) = text_iter.next() {
                return Some((*repository, text));
            }

            self.next_stage();
        }
    }
}

impl FusedIterator for MultiTextIterMut<'_> {}

impl<'a> MultiTextCollection<'a> {
    /// Iterate over mutable references to all items inside a [`MultiTextCollection`], each paired with
    /// their corresponding [`RepositoryName`].
    pub fn iter_mut(&'a mut self) -> MultiTextIterMut<'a> {
        let mut iter = MultiTextIterMut::blank(&mut self.internal);
        iter.next_stage();
        iter
    }
}

/// [Iterator] over owned items inside a [`MultiTextCollection`], each paired with their corresponding
/// [`RepositoryName`].
#[derive(Debug, Clone)]
pub struct MultiTextIntoIter<'a> {
    current: Option<(RepositoryName<'a>, TextIntoIter)>,
    remaining: vec::IntoIter<(RepositoryName<'a>, TextCollection)>,
}

impl<'a> MultiTextIntoIter<'a> {
    /// Construct an (effectively) empty iterator.
    ///
    /// This iterator requires at least one call to [`MultiTextIntoIter::next_stage`] to become useful (i.e. non-empty).
    fn blank(collections: Vec<(RepositoryName<'a>, TextCollection)>) -> Self {
        MultiTextIntoIter {
            current: None,
            remaining: collections.into_iter(),
        }
    }

    /// Extract an item from [`MultiTextIntoIter::remaining`] into [`MultiTextIntoIter::current`] after it has been exhausted.
    fn next_stage(&mut self) {
        debug_assert!(
            self.current.is_none(),
            "next_stage must only be called after current has been exhausted",
        );
        self.current = self
            .remaining
            .next()
            .map(|(repository, collection)| (repository, collection.into_iter()));
    }
}

impl<'a> Iterator for MultiTextIntoIter<'a> {
    type Item = (RepositoryName<'a>, Text);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (repository, text_iter) = self.current.as_mut()?;

            if let Some(text) = text_iter.next() {
                return Some((*repository, text));
            }

            self.next_stage();
        }
    }
}

impl FusedIterator for MultiTextIntoIter<'_> {}

impl<'a> IntoIterator for MultiTextCollection<'a> {
    type Item = (RepositoryName<'a>, Text);
    type IntoIter = MultiTextIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let mut iter = MultiTextIntoIter::blank(self.internal);
        iter.next_stage();
        iter
    }
}
