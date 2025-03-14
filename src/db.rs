//! Common interfaces to interact with database types.

mod pointer;

use arch_pkg_text::value::Name;
use core::convert::Infallible;
use is_type::Is;

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
    fn lookup(&self, name: Name) -> Result<&Self::Querier, Self::Error>;
}

/// Capability to get a mutable reference to a package inside the database.
pub trait LookupMut: PackageDatabase {
    /// Reason for lookup failure.
    type Error;
    /// Get a mutable reference to a package inside the database.
    fn lookup_mut(&mut self, name: Name) -> Result<&mut Self::Querier, Self::Error>;
}

/// Capability to insert a querier into the database.
pub trait Insert: PackageDatabase {
    /// Ejected item on success.
    type Ejection;
    /// Reason for insertion failure.
    type Error;
    /// Insert a querier into the database whose type implemented this trait.
    fn insert(&mut self, querier: Self::Querier) -> Result<Self::Ejection, Self::Error>;
}

/// Utility methods to quickly construct a database by [insertion](Insert).
pub trait Add: Insert<Querier: Sized> + Sized {
    /// Insert a querier into the database whose type implemented [`Insert`].
    fn add(mut self, querier: Self::Querier) -> Result<Self, Self::Error> {
        self.insert(querier)?;
        Ok(self)
    }

    /// Insert a querier into the database whose type implemented [`Insert`] with an [`Infallible`] error.
    fn add_infallible(self, querier: Self::Querier) -> Self
    where
        Self::Error: Is<Type = Infallible>,
    {
        let Ok(db) = self.add(querier).map_err(Is::into_val);
        db
    }
}
impl<Db: Insert<Querier: Sized> + Sized> Add for Db {}
