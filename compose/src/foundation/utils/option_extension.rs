pub(crate) trait OptionThen<T> {
    fn then(self, f: impl FnOnce(T));
}

impl<T> OptionThen<T> for Option<T> {
    fn then(self, f: impl FnOnce(T)) {
        match self {
            Some(value) => f(value),
            None => {}
        }
    }
}