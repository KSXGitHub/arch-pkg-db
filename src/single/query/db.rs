use super::QueryDatabase;
use crate::{Lookup, LookupMut, PackageDatabase};
use arch_pkg_text::value::Name;
use derive_more::{Display, Error};

impl<Querier> PackageDatabase for QueryDatabase<'_, Querier> {
    type Querier = Querier;
}

/// Error type of [`Lookup`] and [`LookupMut`] on [`SingleParsedDatabase`].
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
