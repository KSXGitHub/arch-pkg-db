mod add;
mod get;
mod new;

pub use add::{AddError, NoNameError};

use arch_pkg_text::value::Name;
use std::collections::HashMap;

/// Database with parsed entries.
#[derive(Debug, Clone)]
pub struct SingleParsedDatabase<'a, Querier> {
    internal: HashMap<Name<'a>, Querier>,
}
