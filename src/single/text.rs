mod extend;
mod insert;
mod item;
mod local;
mod misc;
mod new;
mod parse;

pub mod archive;

pub use local::LoadLocalDbError;
pub use parse::TextCollectionParseError;

use derive_more::{Display, From, Into};

/// Owned string type inside [`TextCollection`].
#[derive(Debug, Display, Clone, From, Into, PartialEq, Eq, PartialOrd, Ord)]
pub struct Text(Box<str>);

/// Collection of all `desc` texts from which queriers may access data.
#[derive(Debug, Default, Clone)]
pub struct TextCollection {
    internal: Vec<Text>,
}
