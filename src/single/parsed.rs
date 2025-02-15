mod add;
mod desc_file;
mod get;
mod new;

pub use add::{AddError, NoNameError};
pub use desc_file::DescFile;

use arch_pkg_text::{indexmap::IndexMap, value::Name};

/// Database with parsed entries.
#[derive(Debug, Clone)]
pub struct SingleParsedDatabase<'a, Querier> {
    internal: IndexMap<Name<'a>, DescFile<'a, Querier>>,
}
