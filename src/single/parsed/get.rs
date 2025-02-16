use super::SingleParsedDatabase;
use arch_pkg_text::value::Name;

impl<'a, Querier> SingleParsedDatabase<'a, Querier> {
    /// Get an immutable reference to a `desc` file by package name.
    pub fn get(&self, name: Name<'a>) -> Option<&'_ Querier> {
        self.internal.get(&name)
    }

    /// Get a `desc` file with mutable querier by package name.
    pub fn get_mut(&mut self, name: Name<'a>) -> Option<&mut Querier> {
        self.internal.get_mut(&name)
    }
}
