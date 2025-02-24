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

use super::TextCollection;
use derive_more::{Display, Error};
use mime::SupportedCompressionType;
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
        match SupportedCompressionType::check(bytes) {
            Ok(SupportedCompressionType::Tar) => self.extend_from_tar(bytes)?,
            Ok(SupportedCompressionType::Gzip) => self.extend_from_gz(bytes)?,
            Ok(SupportedCompressionType::Xz) => self.extend_from_xz(bytes)?,
            Err(Some(mime)) => return Err(LoadArchiveError::UnsupportedMimeType(mime)),
            Err(None) => return Err(LoadArchiveError::GetMime),
        }
        Ok(())
    }
}
