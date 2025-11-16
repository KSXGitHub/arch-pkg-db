mod into;
mod utils;

pub use into::{IntoWithParsedVersion, IntoWithRepositoryName};
pub use utils::{WithParsedVersionUtils, WithRepositoryNameUtils};

use crate::value::RepositoryName;
use arch_pkg_text::value::ParsedVersion;
use derive_more::{AsMut, AsRef, Deref, DerefMut};

/// Querier attached to a [`ParsedVersion`].
#[derive(Debug, Clone, Copy, AsRef, AsMut, Deref, DerefMut)]
pub struct WithParsedVersion<'a, Querier> {
    #[deref]
    #[deref_mut]
    querier: Querier,

    #[as_ref(skip)]
    #[as_mut(skip)]
    version: ParsedVersion<'a>,
}

impl<'a, Querier> WithParsedVersion<'a, Querier> {
    /// Pair a querier with a parsed version.
    pub const fn new(querier: Querier, version: ParsedVersion<'a>) -> Self {
        WithParsedVersion { querier, version }
    }

    /// Separate the querier from the parsed version.
    pub fn into_tuple(attached: Self) -> (Querier, ParsedVersion<'a>) {
        (attached.querier, attached.version)
    }
}

/// Querier attached to a [`RepositoryName`].
#[derive(Debug, Clone, Copy, AsRef, AsMut, Deref, DerefMut)]
pub struct WithRepositoryName<'a, Querier> {
    #[deref]
    #[deref_mut]
    querier: Querier,

    #[as_ref(skip)]
    #[as_mut(skip)]
    repository: RepositoryName<'a>,
}

impl<'a, Querier> WithRepositoryName<'a, Querier> {
    /// Pair a querier with a repository name.
    pub const fn new(querier: Querier, repository: RepositoryName<'a>) -> Self {
        WithRepositoryName {
            querier,
            repository,
        }
    }

    /// Separate the querier from the repository name.
    pub fn into_tuple(attached: Self) -> (Querier, RepositoryName<'a>) {
        (attached.querier, attached.repository)
    }
}
