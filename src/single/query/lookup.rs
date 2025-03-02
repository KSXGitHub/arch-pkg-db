use super::QueryDatabase;
use arch_pkg_text::value::Name;

impl<Querier> QueryDatabase<'_, Querier> {
    /// Get an immutable reference to a querier by package name.
    pub fn get(&self, name: Name) -> Option<&Querier> {
        self.internal.get(name.as_str())
    }

    /// Get a mutable reference to a querier by package name.
    pub fn get_mut(&mut self, name: Name) -> Option<&mut Querier> {
        self.internal.get_mut(name.as_str())
    }
}
