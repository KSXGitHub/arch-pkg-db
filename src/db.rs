//! Common interfaces to interact with database types.

mod pointer;

use arch_pkg_text::value::Name;

/// Database of packages.
pub trait PackageDatabase {
    /// Type to query information of a single package inside the database.
    type Querier: ?Sized;
}

/// Capability to get an immutable reference to a package inside the database.
pub trait Lookup: PackageDatabase {
    /// Reason for lookup failure.
    type Error;
    /// Get an immutable reference to a package inside the database.
    fn lookup(&self, name: Name<'_>) -> Result<&'_ Self::Querier, Self::Error>;
}

/// Capability to get a mutable reference to a package inside the database.
pub trait LookupMut: PackageDatabase {
    /// Reason for lookup failure.
    type Error;
    /// Get a mutable reference to a package inside the database.
    fn lookup_mut(&mut self, name: Name<'_>) -> Result<&'_ mut Self::Querier, Self::Error>;
}
