use super::{WithParsedVersion, WithRepositoryName};
use crate::value::RepositoryName;
use arch_pkg_text::value::ParsedVersion;

pub trait IntoWithParsedVersion: Sized {
    /// Attach a parsed version to an object.
    fn with_parsed_version<'a>(self, version: ParsedVersion<'a>) -> WithParsedVersion<'a, Self> {
        WithParsedVersion::new(self, version)
    }
}

impl<Querier: Sized> IntoWithParsedVersion for Querier {}

pub trait IntoWithRepositoryName: Sized {
    /// Attach a repository name to an object.
    fn with_repository_name<'a>(
        self,
        repository: RepositoryName<'a>,
    ) -> WithRepositoryName<'a, Self> {
        WithRepositoryName::new(self, repository)
    }
}

impl<Querier: Sized> IntoWithRepositoryName for Querier {}
