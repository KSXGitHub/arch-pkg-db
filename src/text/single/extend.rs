use super::TextCollection;
use crate::Text;

impl Extend<Text> for TextCollection {
    fn extend<Iter: IntoIterator<Item = Text>>(&mut self, iter: Iter) {
        let iter = iter.into_iter();
        let (cap, _) = iter.size_hint();
        self.internal.reserve(cap);
        for text in iter {
            self.insert(text);
        }
    }
}
