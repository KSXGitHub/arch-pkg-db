mod convert;
mod insert;
mod misc;
mod new;
mod tar;
mod tgz;

pub use tar::LoadTarError;
pub use tgz::LoadTgzError;

/// Collections of all `desc` texts from which queriers may access data.
#[derive(Debug, Default, Clone)]
pub struct Archive {
    internal: Vec<String>,
}
