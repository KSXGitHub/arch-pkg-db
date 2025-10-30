use super::{LoadArchiveError, LoadUncompressedArchiveError};
use crate::{MultiTextCollection, TextCollection, desc::value::RepositoryName};
use derive_more::{Display, Error};
use libflate::gzip::Decoder;
use pipe_trait::Pipe;
use std::io::{self, Read};

/// Error when trying to load data from a gzipped archive.
#[derive(Debug, Display, Error)]
pub enum LoadGzError {
    #[display("Failed to load the gzip archive: {_0}")]
    Gzip(io::Error),
    #[display("Failed to extract data from the internal archive: {_0}")]
    InternalArchive(LoadUncompressedArchiveError),
}

impl TextCollection {
    /// Extract a gzipped archive and add contents from its `desc` files to the text collection.
    pub fn extend_from_gz<Bytes: Read>(&mut self, bytes: Bytes) -> Result<(), LoadGzError> {
        let mut decoder = bytes.pipe(Decoder::new).map_err(LoadGzError::Gzip)?;
        let mut tar = Vec::new();
        decoder.read_to_end(&mut tar).map_err(LoadGzError::Gzip)?;
        self.extend_from_uncompressed_archive(&tar)
            .map_err(LoadGzError::InternalArchive)
    }

    /// Extract a gzipped archive and add contents from its `desc` files to the text collection.
    pub fn add_gz<Bytes: Read>(mut self, bytes: Bytes) -> Result<Self, LoadGzError> {
        self.extend_from_gz(bytes)?;
        Ok(self)
    }

    /// Extract a gzipped archive and add contents from its `desc` files to the text collection.
    pub fn from_gz<Bytes: Read>(bytes: Bytes) -> Result<Self, LoadGzError> {
        TextCollection::new().add_gz(bytes)
    }
}

impl<'a> MultiTextCollection<'a> {
    /// Extract a gzipped archive and add contents from its `desc` files to the multi-collection.
    pub fn extend_from_gz<Bytes: Read>(
        &mut self,
        repository: RepositoryName<'a>,
        bytes: Bytes,
    ) -> Result<(), LoadArchiveError> {
        let collection = TextCollection::from_gz(bytes)?;
        self.insert(repository, collection);
        Ok(())
    }

    /// Extract a gzipped archive and add contents from its `desc` files to the multi-collection.
    pub fn add_gz<Bytes: Read>(
        mut self,
        repository: RepositoryName<'a>,
        bytes: Bytes,
    ) -> Result<Self, LoadArchiveError> {
        self.extend_from_gz(repository, bytes)?;
        Ok(self)
    }

    /// Extract a gzipped archive and add contents from its `desc` files to the multi-collection.
    pub fn from_gz<Bytes: Read>(
        repository: RepositoryName<'a>,
        bytes: Bytes,
    ) -> Result<Self, LoadArchiveError> {
        MultiTextCollection::with_capacity(1).add_gz(repository, bytes)
    }
}

impl From<LoadGzError> for LoadArchiveError {
    fn from(value: LoadGzError) -> Self {
        match value {
            LoadGzError::Gzip(error) => LoadArchiveError::Gzip(error),
            LoadGzError::InternalArchive(error) => LoadArchiveError::InternalArchive(error),
        }
    }
}
