#[derive(Copy, Debug, Clone)]
pub struct Offset<T> {
    pub x: T,
    pub y: T,
}

pub type IntOffset = Offset<i32>;