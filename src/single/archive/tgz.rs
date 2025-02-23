use super::{Archive, LoadTarError};
use derive_more::{Display, Error};
use libflate::gzip::Decoder;
use pipe_trait::Pipe;
use std::io::{self, Read};

/// Error when trying to load data from a `.tar.gz` archive.
#[derive(Debug, Display, Error)]
pub enum LoadTgzError {
    #[display("Failed to load the gzip archive: {_0}")]
    Gzip(io::Error),
    Tar(LoadTarError),
}

impl Archive {
    /// Extract a `.tar.gz` archive and add contents from `desc` files to the archive.
    pub fn extend_from_tgz<Bytes: Read>(
        &mut self,
        raw_tgz_bytes: Bytes,
    ) -> Result<(), LoadTgzError> {
        let tar = raw_tgz_bytes
            .pipe(Decoder::new)
            .map_err(LoadTgzError::Gzip)?;
        self.extend_from_tar(tar).map_err(LoadTgzError::Tar)
    }
}
