//! Miscellaneous items.

mod attachment;

pub use attachment::{Attached, AttachedUtils, IntoAttached};

#[cfg(feature = "parking_lot")]
pub use arch_pkg_text::misc::parking_lot;
pub use arch_pkg_text::misc::{False, StaticBool, True, desc, indexmap, typebool};

pub use is_type;
