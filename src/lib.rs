mod db;
pub use db::{Insert, Lookup, LookupMut, PackageDatabase};

pub mod misc;
pub mod single;

pub use single::QueryDatabase;

pub mod desc {
    //! Fields, queriers, and parser of the text format of `desc` files.
    pub use arch_pkg_text::{
        desc::*,
        parse::{DescParseError, DescParseIssue, ParsedDesc, PartialParseResult},
        value,
    };
}
