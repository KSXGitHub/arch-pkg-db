use super::{Lookup, LookupMut, PackageDatabase};
use crate::attached::Attached;
use arch_pkg_text::value::Name;

impl<Db: PackageDatabase, Attachment> PackageDatabase for Attached<Db, Attachment> {
    type Querier = Db::Querier;
}

impl<Db: Lookup, Attachment> Lookup for Attached<Db, Attachment> {
    type Error = Db::Error;
    fn lookup(&self, name: Name<'_>) -> Result<&'_ Self::Querier, Self::Error> {
        Db::lookup(self, name)
    }
}

impl<Db: LookupMut, Attachment> LookupMut for Attached<Db, Attachment> {
    type Error = Db::Error;
    fn lookup_mut(&mut self, name: Name<'_>) -> Result<&'_ mut Self::Querier, Self::Error> {
        Db::lookup_mut(self, name)
    }
}
