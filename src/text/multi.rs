mod collect;
mod extend;
mod insert;
mod iter;
mod misc;
mod new;
mod parse;

use super::TextCollection;
use crate::desc::value::RepositoryName;

pub use iter::{MultiTextIntoIter, MultiTextIter, MultiTextIterMut};
pub use parse::MultiTextCollectionParseError;

/// Collection of all `desc` texts and repository names from which queriers may access data.
#[derive(Debug, Default, Clone)]
pub struct MultiTextCollection<'a> {
    internal: Vec<(RepositoryName<'a>, TextCollection)>,
}
