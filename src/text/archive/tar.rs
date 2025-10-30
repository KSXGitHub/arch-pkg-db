use super::{LoadArchiveError, LoadUncompressedArchiveError};
use crate::{MultiTextCollection, TextCollection, value::RepositoryName};
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

    /// Traverse a tar archive and add contents from `desc` files to the text collection.
    pub fn add_tar<Bytes: Read>(mut self, bytes: Bytes) -> Result<Self, LoadTarError> {
        self.extend_from_tar(bytes)?;
        Ok(self)
    }

    /// Traverse a tar archive and add contents from `desc` files to the text collection.
    pub fn from_tar<Bytes: Read>(bytes: Bytes) -> Result<Self, LoadTarError> {
        TextCollection::new().add_tar(bytes)
    }
}

impl<'a> MultiTextCollection<'a> {
    /// Extract a tar archive and add contents from its `desc` files to the multi-collection.
    pub fn extend_from_tar<Bytes: Read>(
        &mut self,
        repository: RepositoryName<'a>,
        bytes: Bytes,
    ) -> Result<(), LoadArchiveError> {
        let collection = TextCollection::from_tar(bytes)?;
        self.insert(repository, collection);
        Ok(())
    }

    /// Extract a tar archive and add contents from its `desc` files to the multi-collection.
    pub fn add_tar<Bytes: Read>(
        mut self,
        repository: RepositoryName<'a>,
        bytes: Bytes,
    ) -> Result<Self, LoadArchiveError> {
        self.extend_from_tar(repository, bytes)?;
        Ok(self)
    }

    /// Extract a tar archive and add contents from its `desc` files to the multi-collection.
    pub fn from_tar<Bytes: Read>(
        repository: RepositoryName<'a>,
        bytes: Bytes,
    ) -> Result<Self, LoadArchiveError> {
        MultiTextCollection::with_capacity(1).add_tar(repository, bytes)
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
