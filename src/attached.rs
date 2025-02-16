//! Manipulate metadata attachments.

use core::{mem::replace, ops::Deref};
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

/// Utility trait to create object with attachments.
pub trait IntoAttached: Sized {
    /// Attach metadata to an object.
    fn into_attached<Attachment>(self, attachment: Attachment) -> Attached<Self, Attachment> {
        Attached::new(self, attachment)
    }
}

impl<Main: Sized> IntoAttached for Main {}

/// Methods to interact with [`Attached`].
pub trait AttachedUtils: Sized + sealed::Sealed {
    /// Main data.
    type Main;
    /// Attached metadata.
    type Attachment;

    /// Get an immutable reference to the main data.
    fn main(&self) -> &'_ Self::Main;
    /// Get a mutable reference to the main data.
    fn main_mut(&mut self) -> &'_ mut Self::Main;

    /// Get an immutable reference to the attached metadata.
    fn attachment(&self) -> &'_ Self::Attachment;
    /// Get a mutable reference to the attached data.
    fn attachment_mut(&mut self) -> &'_ mut Self::Attachment;

    /// Separate the main data from the attached metadata.
    fn into_tuple(self) -> (Self::Main, Self::Attachment);

    /// Discard the attached metadata.
    fn into_main(self) -> Self::Main {
        self.into_tuple().0
    }

    /// Get a copy of the attached metadata.
    fn copy_attachment(&self) -> Self::Attachment
    where
        Self::Attachment: Copy,
    {
        *self.attachment()
    }

    /// Get a clone of the attached metadata.
    fn clone_attachment(&self) -> Self::Attachment
    where
        Self::Attachment: Clone,
    {
        self.attachment().clone()
    }

    /// Map the main data.
    fn map<F, Y>(self, f: F) -> Attached<Y, Self::Attachment>
    where
        F: FnOnce(Self::Main) -> Y,
    {
        let (main, attachment) = self.into_tuple();
        f(main).into_attached(attachment)
    }

    /// Convert a reference of a whole pair to an owned pair of references.
    fn as_deref(&self) -> Attached<&'_ Self::Main, &'_ Self::Attachment> {
        self.main().into_attached(self.attachment())
    }

    /// [Copy] the bits of the main data.
    fn copied(&self) -> Attached<<Self::Main as Deref>::Target, &'_ Self::Attachment>
    where
        Self::Main: Deref<Target: Copy>,
    {
        Attached::new(**self.main(), self.attachment())
    }

    /// [Clone] the main data.
    fn cloned(&self) -> Attached<<Self::Main as Deref>::Target, &'_ Self::Attachment>
    where
        Self::Main: Deref<Target: Clone>,
    {
        self.main().deref().clone().into_attached(self.attachment())
    }

    /// [Copy] the bits of the attached metadata.
    fn copied_attachment(&self) -> Attached<&'_ Self::Main, <Self::Attachment as Deref>::Target>
    where
        Self::Attachment: Deref<Target: Copy>,
    {
        self.main().into_attached(**self.attachment())
    }

    /// [Clone] the attached metadata.
    fn cloned_attachment(&self) -> Attached<&'_ Self::Main, <Self::Attachment as Deref>::Target>
    where
        Self::Attachment: Deref<Target: Clone>,
    {
        self.main().into_attached(self.attachment().deref().clone())
    }

    /// Replace the main data.
    fn replace(&mut self, main: Self::Main) -> Self::Main {
        replace(self.main_mut(), main)
    }

    /// Replace the attached metadata.
    fn replace_attachment(&mut self, attachment: Self::Attachment) -> Self::Attachment {
        replace(self.attachment_mut(), attachment)
    }
}

impl<Main, Attachment> sealed::Sealed for Attached<Main, Attachment> {}
impl<Main, Attachment> AttachedUtils for Attached<Main, Attachment> {
    type Main = Main;
    type Attachment = Attachment;

    fn main(&self) -> &'_ Self::Main {
        self
    }

    fn main_mut(&mut self) -> &'_ mut Self::Main {
        self
    }

    fn attachment(&self) -> &'_ Self::Attachment {
        &self.attachment
    }

    fn attachment_mut(&mut self) -> &'_ mut Self::Attachment {
        &mut self.attachment
    }

    fn into_tuple(self) -> (Self::Main, Self::Attachment) {
        Attached::into_tuple(self)
    }
}

mod sealed {
    pub trait Sealed {}
}
