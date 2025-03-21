mod db;
pub use db::{Add, Insert, Lookup, LookupMut, PackageDatabase};

pub mod misc;
pub mod single;

pub use single::{EagerQueryDatabase, MemoQueryDatabase, QueryDatabase};

pub mod desc {
    //! Fields, queriers, and parser of the text format of `desc` files.
    pub use arch_pkg_text::{
        desc::*,
        parse::{
            DescParseError, DescParseIssue, ParseWithIssues, ParsedDesc, PartialParse,
            PartialParseResult,
        },
        value,
    };
}
