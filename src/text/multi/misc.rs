use super::MultiTextCollection;

impl MultiTextCollection<'_> {
    /// Shrink the capacity of the internal data.
    pub fn shrink_to_fit(&mut self) {
        self.internal.shrink_to_fit()
    }

    /// Whether the multi-collection has any element.
    pub fn is_empty(&self) -> bool {
        self.iter().next().is_none()
    }
}
