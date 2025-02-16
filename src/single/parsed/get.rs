use super::SingleParsedDatabase;
use arch_pkg_text::value::Name;

#[expect(
    clippy::needless_lifetimes,
    reason = "it's actually necessary to distinguish between inner and outer lifetimes"
)]
impl<'a, Querier> SingleParsedDatabase<'a, Querier> {
    /// Get an immutable reference to a querier by package name.
    pub fn get(&self, name: Name<'_>) -> Option<&'_ Querier> {
        self.internal.get(name.as_str())
    }

    /// Get a mutable reference to a querier by package name.
    pub fn get_mut(&mut self, name: Name<'_>) -> Option<&'_ mut Querier> {
        self.internal.get_mut(name.as_str())
    }
}
