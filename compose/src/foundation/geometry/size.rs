#[derive(Copy, Debug, Clone, PartialEq)]
pub struct Size<T> {
    packed_value: u64,
    _data: PhantomData<T>
}

pub type IntSize = Size<usize>;