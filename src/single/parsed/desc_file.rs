use std::path::Path;

/// Representation of a `desc` file inside [`SingleParsedDatabase`](super::SingleParsedDatabase).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DescFile<'a, Querier> {
    pub path: &'a Path,
    pub querier: Querier,
}

impl<'a, Querier> DescFile<'a, Querier> {
    /// Get a view of the `desc` file with an immutable querier.
    pub fn as_deref(&self) -> DescFile<'a, &'_ Querier> {
        let DescFile { path, querier } = self;
        let path = *path;
        DescFile { path, querier }
    }

    /// Get a view of the `desc` file with a mutable querier.
    pub fn as_deref_mut(&mut self) -> DescFile<'a, &'_ mut Querier> {
        let DescFile { path, querier } = self;
        let path = *path;
        DescFile { path, querier }
    }
}
