mod convert;
mod insert;
mod load;
mod misc;
mod new;

pub use load::LoadTarError;

/// Collections of all `desc` texts from which queriers may access data.
#[derive(Debug, Default, Clone)]
pub struct Archive {
    internal: Vec<String>,
}
