use core::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};
use derive_more::{Display, From, Into};

/// Owned string type inside [`TextCollection`].
#[derive(Debug, Display, Clone, From, Into, PartialEq, Eq, PartialOrd, Ord)]
pub struct Text(Box<str>);

impl Text {
    /// Extract the string slice containing the entire string.
    pub fn as_str(&self) -> &str {
        self
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        value.into_boxed_str().into()
    }
}

impl From<Text> for String {
    fn from(value: Text) -> Self {
        value.0.into()
    }
}

impl<'a> From<&'a str> for Text {
    fn from(value: &'a str) -> Self {
        value.to_string().into()
    }
}

impl AsRef<str> for Text {
    fn as_ref(&self) -> &str {
        self
    }
}

impl AsMut<str> for Text {
    fn as_mut(&mut self) -> &mut str {
        self
    }
}

impl Borrow<str> for Text {
    fn borrow(&self) -> &str {
        self
    }
}

impl BorrowMut<str> for Text {
    fn borrow_mut(&mut self) -> &mut str {
        self
    }
}

impl Deref for Text {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Text {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
