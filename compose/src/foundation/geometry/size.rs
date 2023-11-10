#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

pub type IntSize = Size<usize>;