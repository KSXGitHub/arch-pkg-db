use derive_more::{AsRef, Deref, Display, From, Into};

/// Name of a repository.
#[derive(Debug, Display, Clone, Copy, AsRef, Deref, From, Into)]
pub struct RepositoryName<'a>(pub &'a str);

impl<'a> RepositoryName<'a> {
    /// Get an immutable reference to the string underneath.
    pub fn as_str(&self) -> &'a str {
        self.0
    }
}
