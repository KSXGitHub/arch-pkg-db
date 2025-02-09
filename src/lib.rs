// #![cfg_attr(not(feature = "std"), no_std)]

mod db;
pub use db::*;

// #[cfg(feature = "std")]
pub use arch_pkg_text::indexmap;
#[cfg(feature = "parking_lot")]
pub use arch_pkg_text::parking_lot;
pub use arch_pkg_text::{desc, parse, value};
pub use flate2;
