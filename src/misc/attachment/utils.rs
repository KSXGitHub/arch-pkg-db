use super::{Attached, IntoAttached};
use core::{mem::replace, ops::Deref};

/// Return type of [`AttachedUtils::flatten`].
type Flattened<Main, Attachment> =
    Attached<<Main as AttachedUtils>::Main, (Attachment, <Main as AttachedUtils>::Attachment)>;

/// Return type of [`AttachedUtils::transpose`].
type Transposed<Main, Attachment> = Attached<
    Attached<<Main as AttachedUtils>::Main, Attachment>,
    <Main as AttachedUtils>::Attachment,
>;

/// Methods to interact with [`Attached`].
pub trait AttachedUtils: Sized + sealed::Sealed {
    /// Main data.
    type Main;
    /// Attached metadata.
    type Attachment;

    /// Get an immutable reference to the main data.
    fn main(&self) -> &Self::Main;
    /// Get a mutable reference to the main data.
    fn main_mut(&mut self) -> &mut Self::Main;

    /// Get an immutable reference to the attached metadata.
    fn attachment(&self) -> &Self::Attachment;
    /// Get a mutable reference to the attached data.
    fn attachment_mut(&mut self) -> &mut Self::Attachment;

    /// Separate the main data from the attached metadata.
    fn into_tuple(self) -> (Self::Main, Self::Attachment);
    /// Get two mutable reference to the main data and the attached metadata.
    fn tuple_mut(&mut self) -> (&mut Self::Main, &mut Self::Attachment);

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

    /// Flatten a pair with nested main.
    fn flatten(self) -> Flattened<Self::Main, Self::Attachment>
    where
        Self::Main: AttachedUtils,
    {
        let (main, attachment1) = self.into_tuple();
        let (main, attachment2) = main.into_tuple();
        main.into_attached((attachment1, attachment2))
    }

    /// Swap inner and outer attachment types of a pair with nested main.
    fn transpose(self) -> Transposed<Self::Main, Self::Attachment>
    where
        Self::Main: AttachedUtils,
    {
        let (main, attachment1) = self.into_tuple();
        let (main, attachment2) = main.into_tuple();
        main.into_attached(attachment1).into_attached(attachment2)
    }

    /// Convert an immutable reference of a whole pair to an owned pair of immutable references.
    fn as_deref(&self) -> Attached<&Self::Main, &Self::Attachment> {
        self.main().into_attached(self.attachment())
    }
    /// Convert a mutable reference of a whole pair to an owned pair of mutable references.
    fn as_deref_mut(&mut self) -> Attached<&mut Self::Main, &mut Self::Attachment> {
        let (main, attachment) = self.tuple_mut();
        main.into_attached(attachment)
    }

    /// [Copy] the bits of the main data.
    fn copied(self) -> Attached<<Self::Main as Deref>::Target, Self::Attachment>
    where
        Self::Main: Deref<Target: Copy>,
    {
        self.map(|attached| *attached)
    }

    /// [Clone] the main data.
    fn cloned(self) -> Attached<<Self::Main as Deref>::Target, Self::Attachment>
    where
        Self::Main: Deref<Target: Clone>,
    {
        self.map(|attached| attached.deref().clone())
    }

    /// [Copy] the bits of the attached metadata.
    fn copied_attachment(self) -> Attached<Self::Main, <Self::Attachment as Deref>::Target>
    where
        Self::Attachment: Deref<Target: Copy>,
    {
        let (main, attachment) = self.into_tuple();
        main.into_attached(*attachment)
    }

    /// [Clone] the attached metadata.
    fn cloned_attachment(self) -> Attached<Self::Main, <Self::Attachment as Deref>::Target>
    where
        Self::Attachment: Deref<Target: Clone>,
    {
        let (main, attachment) = self.into_tuple();
        main.into_attached(attachment.deref().clone())
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

    fn main(&self) -> &Self::Main {
        self
    }

    fn main_mut(&mut self) -> &mut Self::Main {
        self
    }

    fn attachment(&self) -> &Self::Attachment {
        &self.attachment
    }

    fn attachment_mut(&mut self) -> &mut Self::Attachment {
        &mut self.attachment
    }

    fn into_tuple(self) -> (Self::Main, Self::Attachment) {
        Attached::into_tuple(self)
    }

    fn tuple_mut(&mut self) -> (&mut Self::Main, &mut Self::Attachment) {
        let Attached { main, attachment } = self;
        (main, attachment)
    }
}

mod sealed {
    pub trait Sealed {}
}
