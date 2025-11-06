pub mod core;
pub mod extra;

use build_fs_tree::{FileSystemTree, dir, file};
use libflate::gzip;
use lzma_rs::xz_compress;
use std::{io::Write, sync::LazyLock};
use tree_to_archive::BuildTar;

pub static BASH_DB_TREE: LazyLock<FileSystemTree<&str, &str>> = LazyLock::new(|| {
    dir! {
        "bash-5.2.026-2" => dir! {
            "desc" => file!(core::BASH),
        },
        "bash-completion-2.14.0-2" => dir! {
            "desc" => file!(extra::BASH_COMPLETION),
        },
    }
});

pub static BASH_TAR: LazyLock<Vec<u8>> = LazyLock::new(|| BASH_DB_TREE.build_tar().unwrap());

pub static BASH_TGZ: LazyLock<Vec<u8>> = LazyLock::new(|| {
    let mut encoder = gzip::Encoder::new(Vec::new()).unwrap();
    encoder.write_all(BASH_TAR.as_slice()).unwrap();
    encoder.finish().into_result().unwrap()
});

pub static BASH_TXZ: LazyLock<Vec<u8>> = LazyLock::new(|| {
    let mut xz = Vec::new();
    xz_compress(&mut BASH_TAR.as_slice(), &mut xz).unwrap();
    xz
});
