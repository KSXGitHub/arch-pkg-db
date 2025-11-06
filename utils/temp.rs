use derive_more::{AsRef, Deref};
use rand::{Rng, distr::Alphanumeric, rng};
use std::{
    env::temp_dir,
    fs::{create_dir_all, remove_dir_all},
    path::PathBuf,
};

/// Temporary directory that would delete itself upon [`drop`].
#[derive(Debug, AsRef, Deref)]
#[as_ref(forward)]
#[deref(forward)]
pub struct Temp(PathBuf);

impl Temp {
    const SUFFIX_LEN: usize = 15;

    /// Create a new temporary directory and return the handle.
    pub fn new(name_prefix: &str) -> Self {
        use core::fmt::Write;
        let mut name = String::with_capacity(name_prefix.len() + Self::SUFFIX_LEN);
        name.write_str(name_prefix).unwrap();
        for code in rng().sample_iter(Alphanumeric).take(Self::SUFFIX_LEN) {
            name.write_char(code.into()).unwrap();
        }
        let path = temp_dir().join(name);
        create_dir_all(&path).unwrap();
        Temp(path)
    }
}

impl Drop for Temp {
    /// Delete the temporary directory or print a warning on failure.
    fn drop(&mut self) {
        let path = &self.0;
        if let Err(error) = remove_dir_all(path) {
            eprintln!("warning: Failed to delete {path:?}: {error}");
        }
    }
}
