use super::{core::BASH, extra::BASH_COMPLETION};
use crate::temp::Temp;
use build_fs_tree::{Build, FileSystemTree, MergeableFileSystemTree, dir, file};
use libflate::gzip;
use lzma_rs::xz_compress;
use pipe_trait::Pipe;
use std::{io::Write, sync::LazyLock};
use tree_to_archive::BuildTar;

pub static DB_TREE: LazyLock<FileSystemTree<&str, &str>> = LazyLock::new(|| {
    dir! {
        "bash-5.2.026-2" => dir! {
            "desc" => file!(BASH),
        },
        "bash-completion-2.14.0-2" => dir! {
            "desc" => file!(BASH_COMPLETION),
        },
    }
});

pub static TAR: LazyLock<Vec<u8>> = LazyLock::new(|| DB_TREE.build_tar().unwrap());

pub static TGZ: LazyLock<Vec<u8>> = LazyLock::new(|| {
    let mut encoder = gzip::Encoder::new(Vec::new()).unwrap();
    encoder.write_all(TAR.as_slice()).unwrap();
    encoder.finish().into_result().unwrap()
});

pub static TXZ: LazyLock<Vec<u8>> = LazyLock::new(|| {
    let mut xz = Vec::new();
    xz_compress(&mut TAR.as_slice(), &mut xz).unwrap();
    xz
});

pub use DB_TREE as BASH_DB_TREE;
pub use TAR as BASH_TAR;
pub use TGZ as BASH_TGZ;
pub use TXZ as BASH_TXZ;

impl Temp {
    /// Create a local db for bash packages.
    pub fn bash_db() -> Self {
        let temp = Temp::new("testing-bash-local-db-");
        BASH_DB_TREE
            .clone()
            .pipe(MergeableFileSystemTree::from)
            .build(&temp)
            .unwrap();
        temp
    }
}
