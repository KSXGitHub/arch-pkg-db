mod gz;
mod mime;
mod tar;
mod uncompressed;
mod xz;

pub use gz::LoadGzError;
pub use lzma_rs::error::Error as LzmaError;
pub use tar::LoadTarError;
pub use uncompressed::LoadUncompressedArchiveError;
pub use xz::LoadXzError;

use super::{MultiTextCollection, TextCollection};
use crate::desc::value::RepositoryName;
use derive_more::{Display, Error};
use mime::SupportedCompressedArchiveType;
use std::io;

/// Error when trying to load data from an archive.
#[derive(Debug, Display, Error)]
pub enum LoadArchiveError {
    #[display("Cannot detect mime type")]
    GetMime,
    #[display("Mime type not supported: {_0}")]
    UnsupportedMimeType(#[error(not(source))] &'static str),
    Tar(LoadTarError),
    #[display("Failed to load the gzip archive: {_0}")]
    Gzip(io::Error),
    #[display("Failed to load the xz archive: {_0}")]
    Xz(LzmaError),
    #[display("Failed to extract data from the internal archive: {_0}")]
    InternalArchive(LoadUncompressedArchiveError),
}

impl TextCollection {
    /// Detect mime type of an archive, extract it, and add contents from `desc` files to the text collection.
    pub fn extend_from_archive(&mut self, bytes: &[u8]) -> Result<(), LoadArchiveError> {
        match SupportedCompressedArchiveType::check(bytes) {
            Ok(SupportedCompressedArchiveType::Tar) => self.extend_from_tar(bytes)?,
            Ok(SupportedCompressedArchiveType::Gzip) => self.extend_from_gz(bytes)?,
            Ok(SupportedCompressedArchiveType::Xz) => self.extend_from_xz(bytes)?,
            Err(Some(mime)) => return Err(LoadArchiveError::UnsupportedMimeType(mime)),
            Err(None) => return Err(LoadArchiveError::GetMime),
        }
        Ok(())
    }

    /// Detect mime type of an archive, extract it, and add contents from `desc` files to the text collection.
    pub fn add_archive(mut self, bytes: &[u8]) -> Result<Self, LoadArchiveError> {
        self.extend_from_archive(bytes)?;
        Ok(self)
    }

    /// Detect mime type of an archive, extract it, and add contents from `desc` files to the text collection.
    pub fn from_archive(bytes: &[u8]) -> Result<Self, LoadArchiveError> {
        TextCollection::new().add_archive(bytes)
    }
}

impl<'a> MultiTextCollection<'a> {
    /// Detect mime type of an archive, extract it, and add contents from `desc` files to the multi-collection.
    pub fn extend_from_archive(
        &mut self,
        repository: RepositoryName<'a>,
        bytes: &[u8],
    ) -> Result<(), LoadArchiveError> {
        let collection = TextCollection::from_archive(bytes)?;
        self.insert(repository, collection);
        Ok(())
    }

    /// Detect mime type of an archive, extract it, and add contents from `desc` files to the multi-collection.
    pub fn add_archive(
        mut self,
        repository: RepositoryName<'a>,
        bytes: &[u8],
    ) -> Result<Self, LoadArchiveError> {
        self.extend_from_archive(repository, bytes)?;
        Ok(self)
    }

    /// Detect mime type of an archive, extract it, and add contents from `desc` files to the multi-collection.
    pub fn from_archive(
        repository: RepositoryName<'a>,
        bytes: &[u8],
    ) -> Result<Self, LoadArchiveError> {
        MultiTextCollection::with_capacity(1).add_archive(repository, bytes)
    }
}
