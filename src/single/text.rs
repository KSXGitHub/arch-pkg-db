mod convert;
mod insert;
mod local;
mod misc;
mod new;
mod parse;

pub mod archive;

pub use local::LoadLocalDbError;

/// Collections of all `desc` texts from which queriers may access data.
#[derive(Debug, Default, Clone)]
pub struct TextCollection {
    internal: Vec<String>,
}
