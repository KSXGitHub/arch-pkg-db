mod db;
pub use db::{
    Add, Insert, IterEntries, IterEntriesMut, IterNames, IterQueriers, IterQueriersMut, Lookup,
    LookupMut, PackageDatabase,
};

pub mod misc;
pub mod multi;
pub mod single;

pub use multi::{EagerMultiQueryDatabase, MemoMultiQueryDatabase, MultiQueryDatabase};
pub use single::{EagerQueryDatabase, MemoQueryDatabase, QueryDatabase, Text, TextCollection};

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
