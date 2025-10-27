mod extend;
mod insert;
mod iter;
mod misc;
mod new;

use super::RepositoryName;
use crate::misc::Text;

pub use iter::{MultiTextIntoIter, MultiTextIter, MultiTextIterMut};

/// Collection of all `desc` texts and their corresponding [repository names](RepositoryName)
/// from which queriers may access data.
#[derive(Debug, Default, Clone)]
pub struct MultiTextCollection<'a> {
    internal: Vec<(RepositoryName<'a>, Text)>,
}
