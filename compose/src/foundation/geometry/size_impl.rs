use std::ops::{Div, Mul};
use crate::foundation::measure_result::MeasureResult;

use super::{IntSize, Offset, Size, U64ConverterUnsigned};

impl U64ConverterUnsigned for usize {
    fn as_u64(self) -> u64 {
        self as u64
    }

    fn from_u64(data: u64) -> Self {
        data as Self
    }
}

impl<T> Size<T>
    where
        T: U64ConverterUnsigned,
{
    #[inline]
    fn packed_value(x: T, y: T) -> u64 {
        let lhs = x.as_u64();
        let rhs = y.as_u64();
        lhs | (rhs << 32)
    }

    pub fn new(x: T, y: T) -> Size<T> {
        Size {
            packed_value: Self::packed_value(x, y),
            _data: Default::default(),
        }
    }

    pub fn zero() -> Size<T> {
        Size {
            packed_value: Self::packed_value(T::from_u64(0), T::from_u64(0)),
            _data: Default::default(),
        }
    }

    pub fn width(&self) -> T {
        T::from_u64(self.packed_value & 0xffffffff)
    }

    pub fn width_mut(&mut self) -> &mut T {
        unsafe {
            &mut *(self.packed_value as *mut u32 as *mut T)
        }
    }

    pub fn height(&self) -> T {
        T::from_u64((self.packed_value & 0xffffffff00000000) >> 32)
    }

    pub fn height_mut(&mut self) -> &mut T {
        unsafe {
            &mut *((self.packed_value as *mut u32).add(1) as *mut T)
        }
    }

    pub fn center(&self) -> Offset<T> {
        Offset::new(self.width() / T::from_u64(2), self.height() / T::from_u64(2))
    }
}

impl<T> Mul<T> for Size<T>
    where
        T: U64ConverterUnsigned,
{
    type Output = Size<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Size::new(self.width() * rhs, self.height() * rhs)
    }
}

impl<T> Div<T> for Size<T>
    where
        T: U64ConverterUnsigned,
{
    type Output = Size<T>;
    fn div(self, rhs: T) -> Self::Output {
        Size::new(self.width() / rhs, self.height() / rhs)
    }
}

impl<T> Default for Size<T>
    where
        T: U64ConverterUnsigned + Default,
{
    fn default() -> Self {
        Self::new(T::default(), T::default())
    }
}

impl IntSize {
    pub fn as_f32_size(&self) -> Size<f32> {
        Size::new(self.width() as f32, self.height() as f32)
    }
}

impl<T> From<Size<T>> for (T, T) where T: U64ConverterUnsigned {
    fn from(value: Size<T>) -> Self {
        (value.width(), value.height())
    }
}

impl<T> From<(T, T)> for Size<T> where T: U64ConverterUnsigned {
    fn from(value: (T, T)) -> Self {
        Size::new(value.0, value.1)
    }
}

impl From<MeasureResult> for IntSize {
    fn from(value: MeasureResult) -> Self {
        Size::new(value.width, value.height)
    }
}