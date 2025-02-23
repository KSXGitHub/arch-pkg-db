use super::{LoadArchiveError, LoadTarError, LzmaError, TextCollection};
use derive_more::{Display, Error};
use lzma_rs::xz_decompress;
use std::io::{BufReader, Read};

/// Error when trying to load data from a `.tar.xz` archive.
#[derive(Debug, Display, Error)]
pub enum LoadTxzError {
    #[display("Failed to load the xz archive: {_0}")]
    Xz(LzmaError),
    Tar(LoadTarError),
}

impl TextCollection {
    /// Extract a `.tar.xz` archive and add contents from `desc` files to the text collection.
    pub fn extend_from_txz<Bytes: Read>(
        &mut self,
        raw_txz_bytes: Bytes,
    ) -> Result<(), LoadTxzError> {
        let mut buf_reader = BufReader::new(raw_txz_bytes);
        let mut tar = Vec::new();
        xz_decompress(&mut buf_reader, &mut tar).map_err(LoadTxzError::Xz)?;
        self.extend_from_tar(tar.as_slice())
            .map_err(LoadTxzError::Tar)
    }
}

impl From<LoadTxzError> for LoadArchiveError {
    fn from(value: LoadTxzError) -> Self {
        match value {
            LoadTxzError::Xz(error) => LoadArchiveError::Xz(error),
            LoadTxzError::Tar(error) => LoadArchiveError::Tar(error),
        }
    }
}
