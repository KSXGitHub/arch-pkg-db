//! Manipulate metadata attachments.

mod into;
mod utils;

pub use into::IntoAttached;
pub use utils::AttachedUtils;

use derive_more::{AsMut, AsRef, Deref, DerefMut};

/// Pair of main data and attached metadata.
#[derive(Debug, Clone, Copy, AsRef, AsMut, Deref, DerefMut)]
pub struct Attached<Main, Attachment> {
    /// Main data.
    #[deref]
    #[deref_mut]
    main: Main,

    /// Attached metadata.
    #[as_ref(skip)]
    #[as_mut(skip)]
    attachment: Attachment,
}

impl<Main, Attachment> Attached<Main, Attachment> {
    /// Pair `main` with `attachment`.
    pub const fn new(main: Main, attachment: Attachment) -> Self {
        Attached { main, attachment }
    }

    /// Separate `main` from `attachment`.
    pub fn into_tuple(attached: Self) -> (Main, Attachment) {
        (attached.main, attached.attachment)
    }
}
