mod collect;
mod extend;
mod insert;
mod misc;
mod new;
// TODO: mod iter;
// TODO: mod parse;

use super::TextCollection;
use crate::desc::value::RepositoryName;

/// Collection of all `desc` texts and repository names from which queriers may access data.
#[derive(Debug, Default, Clone)]
pub struct MultiTextCollection<'a> {
    internal: Vec<(RepositoryName<'a>, TextCollection)>,
}
