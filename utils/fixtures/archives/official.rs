use crate::{
    fixtures::{
        core::BASH,
        extra::{BASH_COMPLETION, PARALLEL_DISK_USAGE},
    },
    temp::Temp,
};
use build_fs_tree::{Build, FileSystemTree, MergeableFileSystemTree, dir, file};
use libflate::gzip;
use lzma_rs::xz_compress;
use pipe_trait::Pipe;
use std::{io::Write, sync::LazyLock};
use tree_to_archive::BuildTar;

pub static DB_TREE: LazyLock<FileSystemTree<&str, &str>> = LazyLock::new(|| {
    dir! {
        // core
        "bash-5.2.026-2" => dir! {
            "desc" => file!(BASH),
        },

        // extra
        "bash-completion-2.14.0-2" => dir! {
            "desc" => file!(BASH_COMPLETION),
        },
        "parallel-disk-usage-parallel-disk-usage-0.21.1-1" => dir! {
            "desc" => file!(PARALLEL_DISK_USAGE),
        },
    }
});

pub static DB_TEXTS: LazyLock<Vec<&str>> = LazyLock::new(|| {
    DB_TREE
        .dir_content()
        .unwrap()
        .values()
        .flat_map(|subtree| subtree.dir_content())
        .flat_map(|subtree| subtree.get("desc"))
        .flat_map(|desc| desc.file_content())
        .copied()
        .collect()
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

impl Temp {
    /// Create a local db for official packages.
    pub fn official_db() -> Self {
        let temp = Temp::new("testing-official-local-db-");
        DB_TREE
            .clone()
            .pipe(MergeableFileSystemTree::from)
            .build(&temp)
            .unwrap();
        temp
    }
}
