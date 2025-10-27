use super::TextCollection;
use crate::misc::Text;
use derive_more::{Display, Error};
use pipe_trait::Pipe;
use rayon::prelude::*;
use std::{
    fs::{read_dir, read_to_string},
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

/// Error when trying to load data from a local pacman database.
#[derive(Debug, Display, Error)]
pub enum LoadLocalDbError<'a> {
    #[display("Failed to read {path:?} as a directory: {error}")]
    ReadDir {
        #[error(source)]
        error: io::Error,
        path: &'a Path,
    },
    #[display("Failed to read {path:?} as a text file: {error}")]
    ReadFile {
        #[error(source)]
        error: io::Error,
        path: PathBuf,
    },
}

impl TextCollection {
    /// Load data from a local pacman database.
    ///
    /// A local pacman database is a directory usually located at `$ARCH_ROOT/var/lib/pacman/local/`.
    pub fn extend_from_local_db<'path>(
        &mut self,
        local_db_path: &'path Path,
    ) -> Result<(), LoadLocalDbError<'path>> {
        let entries = read_dir(local_db_path).map_err(|error| LoadLocalDbError::ReadDir {
            error,
            path: local_db_path,
        })?;

        for entry in entries {
            let Ok(entry) = entry else { continue };
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            if !file_type.is_dir() {
                continue;
            }
            let file_path = entry.path().join("desc");
            match read_to_string(&file_path) {
                Ok(text) => self.insert(text.into()),
                Err(error) if error.kind() == ErrorKind::NotFound => continue,
                Err(error) => {
                    return Err(LoadLocalDbError::ReadFile {
                        error,
                        path: file_path,
                    });
                }
            };
        }

        Ok(())
    }

    /// Load data from a local pacman database.
    ///
    /// A local pacman database is a directory usually located at `$ARCH_ROOT/var/lib/pacman/local/`.
    pub fn add_local_db(mut self, local_db_path: &'_ Path) -> Result<Self, LoadLocalDbError<'_>> {
        self.extend_from_local_db(local_db_path)?;
        Ok(self)
    }

    /// Load data from a local pacman database.
    ///
    /// A local pacman database is a directory usually located at `$ARCH_ROOT/var/lib/pacman/local/`.
    pub fn from_local_db(local_db_path: &'_ Path) -> Result<Self, LoadLocalDbError<'_>> {
        TextCollection::new().add_local_db(local_db_path)
    }

    /// Load data from a local pacman database in parallel.
    ///
    /// A local pacman database is a directory usually located at `$ARCH_ROOT/var/lib/pacman/local/`.
    pub fn par_extend_from_local_db<'path>(
        &mut self,
        local_db_path: &'path Path,
    ) -> Result<(), LoadLocalDbError<'path>> {
        let texts = local_db_path
            .pipe(read_dir)
            .map_err(|error| LoadLocalDbError::ReadDir {
                error,
                path: local_db_path,
            })?
            .par_bridge()
            .flatten()
            .filter(|entry| {
                entry
                    .file_type()
                    .ok()
                    .map(|file_type| file_type.is_dir())
                    .unwrap_or(false)
            })
            .map(|entry| -> Result<Option<String>, LoadLocalDbError> {
                let file_path = entry.path().join("desc");
                match read_to_string(&file_path) {
                    Ok(text) => Ok(Some(text)),
                    Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
                    Err(error) => Err(LoadLocalDbError::ReadFile {
                        error,
                        path: file_path,
                    }),
                }
            })
            .collect::<Result<Vec<Option<String>>, LoadLocalDbError>>()?
            .into_iter()
            .flatten()
            .map(Text::from);
        self.extend(texts);
        Ok(())
    }

    /// Load data from a local pacman database in parallel.
    ///
    /// A local pacman database is a directory usually located at `$ARCH_ROOT/var/lib/pacman/local/`.
    pub fn par_add_local_db(
        mut self,
        local_db_path: &'_ Path,
    ) -> Result<Self, LoadLocalDbError<'_>> {
        self.par_extend_from_local_db(local_db_path)?;
        Ok(self)
    }

    /// Load data from a local pacman database in parallel.
    ///
    /// A local pacman database is a directory usually located at `$ARCH_ROOT/var/lib/pacman/local/`.
    pub fn par_from_local_db(local_db_path: &Path) -> Result<Self, LoadLocalDbError<'_>> {
        TextCollection::new().par_add_local_db(local_db_path)
    }
}
