use build_fs_tree::{FileSystemTree, dir, file};
use libflate::gzip;
use lzma_rs::xz_compress;
use std::{
    borrow::Cow,
    io::{self, Write},
    sync::LazyLock,
};

pub const BASH: &str = include_str!("fixtures/bash.desc");
pub const BASH_COMPLETION: &str = include_str!("fixtures/bash-completion.desc");

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

fn append_file_to_tar<Writer>(
    builder: &mut tar::Builder<Writer>,
    path: &str,
    data: &[u8],
) -> io::Result<()>
where
    Writer: Write,
{
    let mut header = tar::Header::new_gnu();
    header.set_size(data.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    builder.append_data(&mut header, path, data)
}

fn append_tree_to_tar<Writer: Write>(
    builder: &mut tar::Builder<Writer>,
    path: &str,
    tree: &FileSystemTree<&str, &str>,
) -> io::Result<()> {
    match tree {
        FileSystemTree::File(data) => append_file_to_tar(builder, path, data.as_bytes()),
        FileSystemTree::Directory(children) => {
            for (suffix, subtree) in children {
                let path = match path {
                    "" | "." => Cow::Borrowed(*suffix),
                    _ => Cow::Owned(format!("{path}/{suffix}")),
                };
                append_tree_to_tar(builder, &path, subtree)?;
            }
            Ok(())
        }
    }
}

fn tar_from_tree(tree: &FileSystemTree<&str, &str>) -> io::Result<Vec<u8>> {
    let mut builder = tar::Builder::new(Vec::new());
    append_tree_to_tar(&mut builder, "", tree)?;
    builder.into_inner()
}
