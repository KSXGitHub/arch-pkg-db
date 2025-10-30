//! Collections of `desc` texts from which queriers may access data.

mod item;
mod multi;
mod single;

pub mod archive;
pub mod local;

pub use item::Text;
pub use multi::MultiTextCollection;
pub use single::TextCollection;

pub mod iter {
    pub use super::{
        multi::{MultiTextIntoIter, MultiTextIter, MultiTextIterMut},
        single::{TextIntoIter, TextIter, TextIterMut},
    };
}

pub mod parse {
    pub use super::{multi::MultiTextCollectionParseError, single::TextCollectionParseError};
}
