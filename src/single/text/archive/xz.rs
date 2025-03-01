use super::{LoadArchiveError, LoadUncompressedArchiveError, LzmaError, TextCollection};
use derive_more::{Display, Error};
use lzma_rs::xz_decompress;
use std::io::{BufReader, Read};

/// Error when trying to load data from an xz archive.
#[derive(Debug, Display, Error)]
pub enum LoadXzError {
    #[display("Failed to load the xz archive: {_0}")]
    Xz(LzmaError),
    #[display("Failed to extract data from the internal archive: {_0}")]
    InternalArchive(LoadUncompressedArchiveError),
}

impl TextCollection {
    /// Extract an xz archive and add contents from its `desc` files to the text collection.
    pub fn extend_from_xz<Bytes: Read>(&mut self, bytes: Bytes) -> Result<(), LoadXzError> {
        let mut buf_reader = BufReader::new(bytes);
        let mut tar = Vec::new();
        xz_decompress(&mut buf_reader, &mut tar).map_err(LoadXzError::Xz)?;
        self.extend_from_uncompressed_archive(&tar)
            .map_err(LoadXzError::InternalArchive)
    }

    /// Extract an xz archive and add contents from its `desc` files to the text collection.
    pub fn add_xz<Bytes: Read>(mut self, bytes: Bytes) -> Result<Self, LoadXzError> {
        self.extend_from_xz(bytes)?;
        Ok(self)
    }
}

impl From<LoadXzError> for LoadArchiveError {
    fn from(value: LoadXzError) -> Self {
        match value {
            LoadXzError::Xz(error) => LoadArchiveError::Xz(error),
            LoadXzError::InternalArchive(error) => LoadArchiveError::InternalArchive(error),
        }
    }
}
