use super::TextCollection;
use crate::Text;
use std::{iter::FusedIterator, slice, vec};

/// [Iterator] over immutable references to all items inside a [`TextCollection`].
#[derive(Debug, Clone)]
pub struct TextIter<'a> {
    internal: slice::Iter<'a, Text>,
}

impl<'a> Iterator for TextIter<'a> {
    type Item = &'a Text;

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

impl DoubleEndedIterator for TextIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.internal.next_back()
    }
}

impl ExactSizeIterator for TextIter<'_> {
    fn len(&self) -> usize {
        self.internal.len()
    }
}

impl FusedIterator for TextIter<'_> {}

impl TextCollection {
    /// Iterate over immutable references to the items inside.
    pub fn iter(&self) -> TextIter<'_> {
        TextIter {
            internal: self.internal.iter(),
        }
    }
}

impl<'a> IntoIterator for &'a TextCollection {
    type Item = &'a Text;
    type IntoIter = TextIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// [Iterator] over mutable references to all items inside a [`TextCollection`].
#[derive(Debug)]
pub struct TextIterMut<'a> {
    internal: slice::IterMut<'a, Text>,
}

impl<'a> Iterator for TextIterMut<'a> {
    type Item = &'a mut Text;

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

impl DoubleEndedIterator for TextIterMut<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.internal.next_back()
    }
}

impl ExactSizeIterator for TextIterMut<'_> {
    fn len(&self) -> usize {
        self.internal.len()
    }
}

impl FusedIterator for TextIterMut<'_> {}

impl TextCollection {
    /// Iterate over mutable references to the items inside.
    pub fn iter_mut(&mut self) -> TextIterMut<'_> {
        TextIterMut {
            internal: self.internal.iter_mut(),
        }
    }
}

impl<'a> IntoIterator for &'a mut TextCollection {
    type Item = &'a mut Text;
    type IntoIter = TextIterMut<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// [Iterator] over owned items inside a [`TextCollection`].
#[derive(Debug, Clone)]
pub struct TextIntoIter {
    internal: vec::IntoIter<Text>,
}

impl Iterator for TextIntoIter {
    type Item = Text;

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

impl DoubleEndedIterator for TextIntoIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.internal.next_back()
    }
}

impl ExactSizeIterator for TextIntoIter {
    fn len(&self) -> usize {
        self.internal.len()
    }
}

impl FusedIterator for TextIntoIter {}

impl IntoIterator for TextCollection {
    type Item = Text;
    type IntoIter = TextIntoIter;
    fn into_iter(self) -> Self::IntoIter {
        TextIntoIter {
            internal: self.internal.into_iter(),
        }
    }
}
