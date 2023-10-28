pub(crate) trait WrapWithBox {
    fn wrap_with_box(self) -> Box<Self>;
}

impl<T> WrapWithBox for T {
    #[inline]
    fn wrap_with_box(self) -> Box<Self> {
        Box::new(self)
    }
}