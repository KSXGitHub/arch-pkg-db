use super::TextCollection;
use crate::Text;

impl FromIterator<Text> for TextCollection {
    fn from_iter<Iter: IntoIterator<Item = Text>>(iter: Iter) -> Self {
        TextCollection {
            internal: Vec::from_iter(iter),
        }
    }
}
