use super::{DescFile, SingleParsedDatabase};
use arch_pkg_text::value::Name;
use pipe_trait::Pipe;

impl<'a, Querier> SingleParsedDatabase<'a, Querier> {
    /// Get an immutable reference to a `desc` file by package name.
    pub fn get(&self, name: Name<'a>) -> Option<&'_ DescFile<'a, Querier>> {
        self.internal.get(&name)
    }

    /// Get a `desc` file with immutable querier by package name.
    pub fn get_ref(&self, name: Name<'a>) -> Option<DescFile<'a, &'_ Querier>> {
        self.get(name)?.as_deref().pipe(Some)
    }

    /// Get a `desc` file with mutable querier by package name.
    pub fn get_mut(&mut self, name: Name<'a>) -> Option<DescFile<'a, &'_ mut Querier>> {
        self.internal.get_mut(&name)?.as_deref_mut().pipe(Some)
    }
}
