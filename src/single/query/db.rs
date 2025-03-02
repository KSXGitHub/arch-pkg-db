use super::QueryDatabase;
use crate::PackageDatabase;

impl<Querier> PackageDatabase for QueryDatabase<'_, Querier> {
    type Querier = Querier;
}
