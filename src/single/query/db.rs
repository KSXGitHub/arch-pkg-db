use super::{AddError, QueryDatabase};
use crate::{Insert, Lookup, LookupMut, PackageDatabase};
use arch_pkg_text::{desc::QueryMut, value::Name};
use derive_more::{Display, Error};

impl<Querier> PackageDatabase for QueryDatabase<'_, Querier> {
    type Querier = Querier;
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

impl<'a, Querier> Insert for QueryDatabase<'a, Querier>
where
    Querier: QueryMut<'a>,
{
    type Ejection = Option<Querier>;
    type Error = AddError<Querier>;
    fn insert(&mut self, querier: Self::Querier) -> Result<Self::Ejection, Self::Error> {
        self.insert(querier)
    }
}
