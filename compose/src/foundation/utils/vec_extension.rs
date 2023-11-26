pub(crate) trait VecExtension<T> {
    fn is_not_empty(&self) -> bool;
}

impl<T> VecExtension<T> for Vec<T> {
    #[inline]
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}