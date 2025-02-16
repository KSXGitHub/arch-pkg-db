mod db;

pub mod attached;
pub mod single;

pub use db::{Lookup, LookupMut, PackageDatabase};
pub use single::SingleParsedDatabase;

pub use arch_pkg_text::indexmap;
#[cfg(feature = "parking_lot")]
pub use arch_pkg_text::parking_lot;
pub use arch_pkg_text::{desc, parse, value};
