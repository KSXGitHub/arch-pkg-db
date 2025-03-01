use libflate::gzip;
use lzma_rs::xz_compress;
use std::{
    io::{self, Write},
    sync::LazyLock,
};

pub const BASH: &str = include_str!("fixtures/bash.desc");
pub const BASH_COMPLETION: &str = include_str!("fixtures/bash-completion.desc");

pub static BASH_TAR: LazyLock<Vec<u8>> = LazyLock::new(|| {
    let mut builder = tar::Builder::new(Vec::new());

    append_file_to_tar(&mut builder, "bash-5.2.026-2/desc", BASH.as_bytes()).unwrap();
    append_file_to_tar(
        &mut builder,
        "bash-completion-2.14.0-2/desc",
        BASH_COMPLETION.as_bytes(),
    )
    .unwrap();

    builder.into_inner().unwrap()
});

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
