use crate::archive::tar_from_tree;
use build_fs_tree::{FileSystemTree, dir, file};
use libflate::gzip;
use lzma_rs::xz_compress;
use std::{io::Write, sync::LazyLock};

pub static BASH: &str = include_str!("fixtures/bash.desc");
pub static BASH_COMPLETION: &str = include_str!("fixtures/bash-completion.desc");

pub static BASH_DB_TREE: LazyLock<FileSystemTree<&str, &str>> = LazyLock::new(|| {
    dir! {
        "bash-5.2.026-2" => dir! {
            "desc" => file!(BASH),
        },
        "bash-completion-2.14.0-2" => dir! {
            "desc" => file!(BASH_COMPLETION),
        },
    }
});

pub static BASH_TAR: LazyLock<Vec<u8>> = LazyLock::new(|| tar_from_tree(&BASH_DB_TREE).unwrap());

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
