mod extend;
mod insert;
mod iter;
mod misc;
mod new;
mod parse;

use super::RepositoryName;
use crate::misc::Text;

pub use iter::{MultiTextIntoIter, MultiTextIter, MultiTextIterMut};
pub use parse::MultiTextCollectionParseError;

/// Collection of all `desc` texts and their corresponding [repository names](RepositoryName)
/// from which queriers may access data.
#[derive(Debug, Default, Clone)]
pub struct MultiTextCollection<'a> {
    internal: Vec<(RepositoryName<'a>, Text)>,
}
