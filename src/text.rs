mod item;
mod multi;
mod single;

pub mod archive;
pub mod local;

pub use item::Text;
pub use multi::{MultiTextCollection, MultiTextIntoIter, MultiTextIter, MultiTextIterMut};
pub use single::{TextCollection, TextIntoIter, TextIter, TextIterMut};
