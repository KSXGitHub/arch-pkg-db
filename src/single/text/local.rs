use super::TextCollection;
use derive_more::{Display, Error};
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
}
