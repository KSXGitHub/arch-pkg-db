use super::Attached;

/// Utility trait to create object with attachments.
pub trait IntoAttached: Sized {
    /// Attach metadata to an object.
    fn into_attached<Attachment>(self, attachment: Attachment) -> Attached<Self, Attachment> {
        Attached::new(self, attachment)
    }
}

impl<Main: Sized> IntoAttached for Main {}
