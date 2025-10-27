mod extend;
mod insert;
mod iter;
mod local;
mod misc;
mod new;
mod parse;

pub mod archive;

pub use iter::{TextIntoIter, TextIter, TextIterMut};
pub use local::LoadLocalDbError;
pub use parse::TextCollectionParseError;

use crate::misc::Text;

/// Collection of all `desc` texts from which queriers may access data.
#[derive(Debug, Default, Clone)]
pub struct TextCollection {
    internal: Vec<Text>,
}
