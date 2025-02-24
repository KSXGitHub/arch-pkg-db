mod add;
mod db;
mod get;
mod new;

pub use add::{AddError, NoNameError};
pub use db::LookupError;

use std::collections::HashMap;

/// Database to lookup queriers from their package names.
#[derive(Debug, Clone)]
pub struct QueryDatabase<'a, Querier> {
    /// Map package names to their queriers.
    internal: HashMap<&'a str, Querier>,
}
