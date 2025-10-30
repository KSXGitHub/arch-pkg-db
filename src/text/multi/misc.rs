use super::MultiTextCollection;

impl MultiTextCollection<'_> {
    /// Shrink the capacity of the internal data.
    pub fn shrink_to_fit(&mut self) {
        self.internal.shrink_to_fit()
    }

    /// The number of [`Text`](crate::Text)s within the multi-collection.
    pub fn len(&self) -> usize {
        self.internal.len()
    }

    /// Whether the multi-collection is empty.
    pub fn is_empty(&self) -> bool {
        self.internal.is_empty()
    }
}
