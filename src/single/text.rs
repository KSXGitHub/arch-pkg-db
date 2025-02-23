mod archive;
mod convert;
mod insert;
mod local;
mod misc;
mod new;
mod tar;
mod tgz;
mod txz;

pub use archive::LoadArchiveError;
pub use local::LoadLocalDbError;
pub use lzma_rs::error::Error as LzmaError;
pub use tar::LoadTarError;
pub use tgz::LoadTgzError;
pub use txz::LoadTxzError;

/// Collections of all `desc` texts from which queriers may access data.
#[derive(Debug, Default, Clone)]
pub struct TextCollection {
    internal: Vec<String>,
}
