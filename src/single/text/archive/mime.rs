use strum::{AsRefStr, EnumString};

/// Mime type of a supported archive format.
#[derive(Clone, AsRefStr, EnumString)]
#[strum(use_phf)]
pub enum SupportedArchiveType {
    #[strum(serialize = "application/x-tar")]
    Tar,
    #[strum(serialize = "application/gzip")]
    Gzip,
    #[strum(serialize = "application/x-xz")]
    Xz,
}
