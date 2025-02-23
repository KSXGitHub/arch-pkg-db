use strum::{AsRefStr, Display, EnumString};

/// Mime type of a supported archive format.
#[derive(Debug, Display, Clone, Copy, AsRefStr, EnumString)]
pub enum SupportedArchiveType {
    #[strum(serialize = "application/x-tar")]
    Tar,
    #[strum(serialize = "application/gzip")]
    Gzip,
    #[strum(serialize = "application/x-xz")]
    Xz,
}
