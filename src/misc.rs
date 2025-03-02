//! Miscellaneous items.

#[cfg(feature = "parking_lot")]
pub use arch_pkg_text::misc::parking_lot;
pub use arch_pkg_text::misc::{False, StaticBool, True, desc, indexmap, typebool};

/// Asserting one type to be the exact same as another.
#[diagnostic::on_unimplemented(message = "Expecting type `{X}`, got type `{Self}`")]
pub trait IsType<X: ?Sized> {
    fn cast(self) -> X
    where
        X: Sized;
    fn cast_ref(&self) -> &X;
    fn cast_mut(&mut self) -> &mut X;
}

#[diagnostic::do_not_recommend]
impl<X: ?Sized> IsType<X> for X {
    fn cast(self) -> X
    where
        X: Sized,
    {
        self
    }

    fn cast_ref(&self) -> &X {
        self
    }

    fn cast_mut(&mut self) -> &mut X {
        self
    }
}
