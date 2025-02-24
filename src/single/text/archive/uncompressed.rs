use super::{mime::SupportedUncompressedArchiveType, tar::LoadTarError};
use crate::single::TextCollection;
use derive_more::{Display, Error};

/// Error when trying to load data from an uncompressed archive.
#[derive(Debug, Display, Error)]
pub enum LoadUncompressedArchiveError {
    #[display("Cannot detect mime type")]
    GetMime,
    #[display("Mime type not supported: {_0}")]
    UnsupportedMimeType(#[error(not(source))] &'static str),
    Tar(LoadTarError),
}

impl TextCollection {
    /// Detect the mime type of an uncompressed archive, traverse it, and add contents from
    /// its `desc` files to the text collection.
    pub(super) fn extend_from_uncompressed_archive(
        &mut self,
        bytes: &[u8],
    ) -> Result<(), LoadUncompressedArchiveError> {
        match SupportedUncompressedArchiveType::check(bytes) {
            Ok(SupportedUncompressedArchiveType::Tar) => self
                .extend_from_tar(bytes)
                .map_err(LoadUncompressedArchiveError::Tar),
            Err(Some(mime)) => Err(LoadUncompressedArchiveError::UnsupportedMimeType(mime)),
            Err(None) => Err(LoadUncompressedArchiveError::GetMime),
        }
    }
}
