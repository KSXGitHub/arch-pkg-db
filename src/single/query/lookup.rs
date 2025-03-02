use super::QueryDatabase;
use crate::{Lookup, LookupMut};
use arch_pkg_text::value::Name;
use derive_more::{Display, Error};

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

/// Error type of [`Lookup`] and [`LookupMut`] on [`QueryDatabase`].
#[derive(Debug, Display, Error)]
#[display("Name matches no entry")]
#[non_exhaustive]
pub struct LookupError;

impl<Querier> Lookup for QueryDatabase<'_, Querier> {
    type Error = LookupError;
    fn lookup(&self, name: Name<'_>) -> Result<&'_ Self::Querier, Self::Error> {
        self.get(name).ok_or(LookupError)
    }
}

impl<Querier> LookupMut for QueryDatabase<'_, Querier> {
    type Error = LookupError;
    fn lookup_mut(&mut self, name: Name<'_>) -> Result<&'_ mut Self::Querier, Self::Error> {
        self.get_mut(name).ok_or(LookupError)
    }
}
