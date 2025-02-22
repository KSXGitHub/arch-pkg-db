mod db;
pub use db::{Insert, Lookup, LookupMut, PackageDatabase};

pub mod misc;
pub mod single;

pub use single::SingleParsedDatabase;

pub use arch_pkg_text::{desc, parse, value};
