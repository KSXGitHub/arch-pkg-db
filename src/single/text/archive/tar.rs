use super::{LoadArchiveError, LoadUncompressedArchiveError};
use crate::single::TextCollection;
use derive_more::{Display, Error};
use pipe_trait::Pipe;
use std::{
    ffi::OsStr,
    io::{self, Read},
};

/// Error when trying to load data from a tar archive.
#[derive(Debug, Display, Error)]
#[display("Failed to read the tar archive: {_0}")]
pub struct LoadTarError(io::Error);

impl TextCollection {
    /// Traverse a tar archive and add contents from `desc` files to the text collection.
    pub fn extend_from_tar<Bytes: Read>(&mut self, bytes: Bytes) -> Result<(), LoadTarError> {
        let mut tar = tar::Archive::new(bytes);
        let entries = tar.entries().map_err(LoadTarError)?;

        for entry in entries {
            let mut entry = entry.map_err(LoadTarError)?;
            let path = entry.path().map_err(LoadTarError)?;
            let file_name = path.file_name().and_then(OsStr::to_str);
            if file_name != Some("desc") {
                continue;
            }
            let mut text = entry
                .header()
                .size()
                .unwrap_or(0)
                .pipe(usize::try_from)
                .unwrap_or(0)
                .pipe(String::with_capacity);
            entry.read_to_string(&mut text).map_err(LoadTarError)?;
            self.insert(text.into());
        }

        Ok(())
    }
}

impl From<LoadTarError> for LoadUncompressedArchiveError {
    fn from(value: LoadTarError) -> Self {
        LoadUncompressedArchiveError::Tar(value)
    }
}

impl From<LoadTarError> for LoadArchiveError {
    fn from(value: LoadTarError) -> Self {
        LoadArchiveError::Tar(value)
    }
}
