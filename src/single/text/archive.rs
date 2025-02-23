mod mime;

use super::{LoadTarError, LzmaError, TextCollection};
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
        match SupportedArchiveType::try_from(mime_type) {
            Ok(SupportedArchiveType::Tar) => self.extend_from_tar(raw_archive_bytes)?,
            Ok(SupportedArchiveType::Gzip) => self.extend_from_tgz(raw_archive_bytes)?,
            Ok(SupportedArchiveType::Xz) => self.extend_from_txz(raw_archive_bytes)?,
            Err(_) => return Err(LoadArchiveError::UnsupportedMimeType(mime_type)),
        }
        Ok(())
    }
}
