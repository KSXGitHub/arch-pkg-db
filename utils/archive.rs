use build_fs_tree::FileSystemTree;
use std::{
    borrow::Cow,
    io::{self, Write},
};

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

pub fn tar_from_tree(tree: &FileSystemTree<&str, &str>) -> io::Result<Vec<u8>> {
    let mut builder = tar::Builder::new(Vec::new());
    append_tree_to_tar(&mut builder, "", tree)?;
    builder.into_inner()
}
