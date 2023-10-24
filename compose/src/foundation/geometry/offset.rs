use std::marker::PhantomData;

#[derive(Copy, Debug, Clone)]
pub struct Offset<T> {
    packed_value: u64,
    _data: PhantomData<T>
}

pub type IntOffset = Offset<i32>;