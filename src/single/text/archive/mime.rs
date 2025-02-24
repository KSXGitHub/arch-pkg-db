use pipe_trait::Pipe;
use strum::{AsRefStr, EnumString};

/// Mime type of a supported archive format.
#[derive(Clone, AsRefStr, EnumString)]
#[strum(use_phf)]
pub enum SupportedArchiveType {
    #[strum(serialize = "application/x-tar")]
    Tar,
}

impl SupportedArchiveType {
    /// Get mime type of an archive.
    pub(super) fn check(bytes: &[u8]) -> Result<Self, Option<&'static str>> {
        let mime = bytes.pipe(infer::get).ok_or(None)?.mime_type();
        mime.parse().map_err(|_| Some(mime))
    }
}

/// Mime type of a supported compression format.
#[derive(Clone, AsRefStr, EnumString)]
#[strum(use_phf)]
pub enum SupportedCompressionType {
    #[strum(serialize = "application/x-tar")]
    Tar,
    #[strum(serialize = "application/gzip")]
    Gzip,
    #[strum(serialize = "application/x-xz")]
    Xz,
}

impl SupportedCompressionType {
    /// Get mime type of a compressed file.
    pub(super) fn check(bytes: &[u8]) -> Result<Self, Option<&'static str>> {
        let mime = bytes.pipe(infer::get).ok_or(None)?.mime_type();
        mime.parse().map_err(|_| Some(mime))
    }
}
