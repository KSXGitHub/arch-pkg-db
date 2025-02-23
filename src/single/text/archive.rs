mod mime;

use super::{LoadTarError, LoadTgzError, LoadTxzError, LzmaError, TextCollection};
use derive_more::{Display, Error};
use mime::SupportedArchiveType;
use std::io;

/// Error when trying to load data from an archive.
#[derive(Debug, Display, Error)]
pub enum LoadArchiveError {
    #[display("Cannot detect mime type")]
    GetMime,
    #[display("Mime type not supported: {_0}")]
    UnsupportedMimeType(#[error(not(source))] &'static str),
    #[display("Failed to load the gzip archive: {_0}")]
    Gzip(io::Error),
    #[display("Failed to load the xz archive: {_0}")]
    Xz(LzmaError),
    Tar(LoadTarError),
}

impl TextCollection {
    /// Detect mime type of an archive, extract it, and add contents from `desc` files to the text collection.
    pub fn extend_from_archive(
        &mut self,
        raw_archive_bytes: &[u8],
    ) -> Result<(), LoadArchiveError> {
        let mime_type = infer::get(raw_archive_bytes)
            .ok_or(LoadArchiveError::GetMime)?
            .mime_type();
        Err(match SupportedArchiveType::try_from(mime_type) {
            Ok(SupportedArchiveType::Tar) => match self.extend_from_tar(raw_archive_bytes) {
                Ok(()) => return Ok(()),
                Err(error) => LoadArchiveError::Tar(error),
            },
            Ok(SupportedArchiveType::Gzip) => match self.extend_from_tgz(raw_archive_bytes) {
                Ok(()) => return Ok(()),
                Err(LoadTgzError::Tar(error)) => LoadArchiveError::Tar(error),
                Err(LoadTgzError::Gzip(error)) => LoadArchiveError::Gzip(error),
            },
            Ok(SupportedArchiveType::Xz) => match self.extend_from_txz(raw_archive_bytes) {
                Ok(()) => return Ok(()),
                Err(LoadTxzError::Tar(error)) => LoadArchiveError::Tar(error),
                Err(LoadTxzError::Xz(error)) => LoadArchiveError::Xz(error),
            },
            Err(_) => LoadArchiveError::UnsupportedMimeType(mime_type),
        })
    }
}
