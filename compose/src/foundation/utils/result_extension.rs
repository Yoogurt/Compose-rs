pub trait ResultExtension<T, E> {
    fn map_err_to_box(self) -> Result<T, Box<E>>;
}

impl<T, E> ResultExtension<T, E> for Result<T, E> {
    fn map_err_to_box(self) -> Result<T, Box<E>> {
        self.map_err(|e| Box::new(e))
    }
}