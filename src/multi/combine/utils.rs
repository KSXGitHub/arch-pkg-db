use super::{IntoWithParsedVersion, IntoWithRepositoryName, WithParsedVersion, WithRepositoryName};
use crate::value::RepositoryName;
use arch_pkg_text::value::ParsedVersion;

/// Methods to interact with [`WithParsedVersion`].
pub trait WithParsedVersionUtils<'a>: Sized + sealed::Sealed {
    /// The type of the querier.
    type Querier;

    /// Get an immutable reference to the querier.
    fn querier(&self) -> &Self::Querier;
    /// Get a mutable reference to the querier.
    fn querier_mut(&mut self) -> &mut Self::Querier;

    /// Get the attached parsed version.
    fn parsed_version(&self) -> ParsedVersion<'a>;
    /// Get a mutable reference to the attached parsed version.
    fn parsed_version_mut(&mut self) -> &mut ParsedVersion<'a>;

    /// Separate the querier from the attached parsed version.
    fn into_tuple(self) -> (Self::Querier, ParsedVersion<'a>);
    /// Get two mutable references to the querier and the attached parsed version.
    fn tuple_mut(&mut self) -> (&mut Self::Querier, &mut ParsedVersion<'a>);

    /// Discard the attached parsed version.
    fn into_querier(self) -> Self::Querier {
        self.into_tuple().0
    }

    /// Create a wrapper of an immutable reference to the querier.
    fn to_ref(&self) -> WithParsedVersion<'a, &Self::Querier> {
        self.querier().with_parsed_version(self.parsed_version())
    }

    /// Create a wrapper of a mutable reference to the querier.
    fn to_ref_mut(&mut self) -> WithParsedVersion<'a, &mut Self::Querier> {
        let parsed_version = self.parsed_version();
        self.querier_mut().with_parsed_version(parsed_version)
    }
}

impl<Querier> sealed::Sealed for WithParsedVersion<'_, Querier> {}
impl<'a, Querier> WithParsedVersionUtils<'a> for WithParsedVersion<'a, Querier> {
    type Querier = Querier;

    fn querier(&self) -> &Self::Querier {
        &self.querier
    }

    fn querier_mut(&mut self) -> &mut Self::Querier {
        &mut self.querier
    }

    fn parsed_version(&self) -> ParsedVersion<'a> {
        self.version
    }

    fn parsed_version_mut(&mut self) -> &mut ParsedVersion<'a> {
        &mut self.version
    }

    fn into_tuple(self) -> (Self::Querier, ParsedVersion<'a>) {
        WithParsedVersion::into_tuple(self)
    }

    fn tuple_mut(&mut self) -> (&mut Self::Querier, &mut ParsedVersion<'a>) {
        let WithParsedVersion { querier, version } = self;
        (querier, version)
    }
}

/// Methods to interact with [`WithRepositoryName`].
pub trait WithRepositoryNameUtils<'a>: Sized + sealed::Sealed {
    /// The type of the querier.
    type Querier;

    /// Get an immutable reference to the querier.
    fn querier(&self) -> &Self::Querier;
    /// Get a mutable reference to the querier.
    fn querier_mut(&mut self) -> &mut Self::Querier;

    /// Get the attached repository name.
    fn repository_name(&self) -> RepositoryName<'a>;
    /// Get a mutable reference to the attached repository name.
    fn repository_name_mut(&mut self) -> &mut RepositoryName<'a>;

    /// Separate the querier from the attached repository name.
    fn into_tuple(self) -> (Self::Querier, RepositoryName<'a>);
    /// Get two mutable references to the querier and the attached repository name.
    fn tuple_mut(&mut self) -> (&mut Self::Querier, &mut RepositoryName<'a>);

    /// Discard the attached repository name.
    fn into_querier(self) -> Self::Querier {
        self.into_tuple().0
    }

    /// Create a wrapper of an immutable reference to the querier.
    fn to_ref(&self) -> WithRepositoryName<'a, &Self::Querier> {
        self.querier().with_repository_name(self.repository_name())
    }

    /// Create a wrapper of a mutable reference to the querier.
    fn to_ref_mut(&mut self) -> WithRepositoryName<'a, &mut Self::Querier> {
        let repository = self.repository_name();
        self.querier_mut().with_repository_name(repository)
    }
}

impl<Querier> sealed::Sealed for WithRepositoryName<'_, Querier> {}
impl<'a, Querier> WithRepositoryNameUtils<'a> for WithRepositoryName<'a, Querier> {
    type Querier = Querier;

    fn querier(&self) -> &Self::Querier {
        &self.querier
    }

    fn querier_mut(&mut self) -> &mut Self::Querier {
        &mut self.querier
    }

    fn repository_name(&self) -> RepositoryName<'a> {
        self.repository
    }

    fn repository_name_mut(&mut self) -> &mut RepositoryName<'a> {
        &mut self.repository
    }

    fn into_tuple(self) -> (Self::Querier, RepositoryName<'a>) {
        WithRepositoryName::into_tuple(self)
    }

    fn tuple_mut(&mut self) -> (&mut Self::Querier, &mut RepositoryName<'a>) {
        let WithRepositoryName {
            querier,
            repository,
        } = self;
        (querier, repository)
    }
}

mod sealed {
    pub trait Sealed {}
}
