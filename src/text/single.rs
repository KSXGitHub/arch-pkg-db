mod extend;
mod insert;
mod iter;
mod misc;
mod new;
mod parse;

/// Collection of all `desc` texts from which queriers may access data.
#[derive(Debug, Default, Clone)]
pub struct TextCollection {
    internal: Vec<crate::Text>,
}
