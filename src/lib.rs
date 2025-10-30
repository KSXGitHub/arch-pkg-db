pub mod misc;
pub mod multi;
pub mod single;
pub mod text;

pub use multi::{EagerMultiQueryDatabase, MemoMultiQueryDatabase, MultiQueryDatabase};
pub use single::{EagerQueryDatabase, MemoQueryDatabase, QueryDatabase};
pub use text::{MultiTextCollection, Text, TextCollection};

pub mod desc {
    //! Fields, queriers, and parser of the text format of `desc` files.
    pub use arch_pkg_text::{
        desc::*,
        parse::{
            DescParseError, DescParseIssue, ParseWithIssues, ParsedDesc, PartialParse,
            PartialParseResult,
        },
    };

    pub mod value {
        //! Value types used by the database.
        mod repository;

        pub use arch_pkg_text::value::*;
        pub use repository::RepositoryName;
    }
}
