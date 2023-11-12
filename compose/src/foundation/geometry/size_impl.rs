use std::ops::{Div, Mul};

use crate::foundation::measure_result::MeasureResult;

use super::{IntSize, Offset, Size};

impl<T> Size<T> {
    pub fn new(x: T, y: T) -> Size<T> {
        Size {
            width: x,
            height: y,
        }
    }

    pub fn zero() -> Size<T> where T: Default {
        Size {
            width: T::default(),
            height: T::default(),
        }
    }
}

impl<T> Mul<T> for Size<T> where T: Mul<Output=T> + Copy
{
    type Output = Size<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Size::new(self.width * rhs, self.height * rhs)
    }
}

impl<T> Div<T> for Size<T>where T: Div<Output=T> + Copy
{
    type Output = Size<T>;
    fn div(self, rhs: T) -> Self::Output {
        Size::new(self.width / rhs, self.height / rhs)
    }
}

impl<T> Default for Size<T> where T: Default,
{
    fn default() -> Self {
        Self::new(T::default(), T::default())
    }
}

impl IntSize {
    pub fn as_f32_size(&self) -> Size<f32> {
        Size::new(self.width as f32, self.height as f32)
    }
}

impl<T> From<Size<T>> for (T, T) {
    fn from(value: Size<T>) -> Self {
        (value.width, value.height)
    }
}

impl<T> From<(T, T)> for Size<T> {
    fn from(value: (T, T)) -> Self {
        Size::new(value.0, value.1)
    }
}

impl From<MeasureResult> for IntSize {
    fn from(value: MeasureResult) -> Self {
        Size::new(value.width, value.height)
    }
}

impl Size<u32> {
    pub fn center(&self) -> Offset<u32> {
        Offset::new(self.width / 2, self.height / 2)
    }
}

impl Size<f32> {
    pub fn center(&self) -> Offset<f32> {
        Offset::new(self.width / 2f32, self.height / 2f32)
    }
}