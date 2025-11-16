mod collect;
mod extend;
mod insert;
mod iter;
mod misc;
mod new;
mod parse;

pub use iter::{TextIntoIter, TextIter, TextIterMut};
pub use parse::TextCollectionParseError;

/// Collection of all `desc` texts from which queriers may access data.
#[derive(Debug, Default, Clone)]
pub struct TextCollection {
    internal: Vec<crate::Text>,
}
