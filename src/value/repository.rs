use derive_more::{AsRef, Deref, Display, From, Into};

/// Name of a repository.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, AsRef, Deref, From, Into)]
pub struct RepositoryName<'a>(pub &'a str);

impl<'a> RepositoryName<'a> {
    /// Get an immutable reference to the string underneath.
    pub fn as_str(&self) -> &'a str {
        self.0
    }

    /// Whether this name is a valid name.
    pub fn is_valid(&self) -> bool {
        let valid_char = |char| matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | '.');
        !self.is_empty() && self.chars().all(valid_char)
    }
}
