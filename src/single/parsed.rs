mod add;
mod get;
mod new;

pub use add::{AddError, NoNameError};

use std::collections::HashMap;

/// Database with parsed entries.
#[derive(Debug, Clone)]
pub struct SingleParsedDatabase<'a, Querier> {
    /// Map package names to their queriers.
    internal: HashMap<&'a str, Querier>,
}
