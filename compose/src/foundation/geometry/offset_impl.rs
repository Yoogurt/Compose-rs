use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use lazy_static::lazy_static;

use super::{IntOffset, Offset};

impl<T> Offset<T>
{
    pub fn new(x: T, y: T) -> Offset<T> {
        Offset {
            x,
            y,
        }
    }

    pub fn zero() -> Offset<T> where T: Default {
        Offset {
            x: T::default(),
            y: T::default(),
        }
    }
}

impl<T> Neg for Offset<T>
    where
        T: Neg<Output=T>,
{
    type Output = Offset<T>;
    fn neg(self) -> Self::Output {
        Offset::new(-self.x, -self.y)
    }
}

impl<T> Sub for Offset<T>
    where
        T: Sub<Output=T>,
{
    type Output = Offset<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Offset::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> Add for Offset<T> where T: Add<Output=T>
{
    type Output = Offset<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Offset::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> Mul<T> for Offset<T> where T: Mul<Output=T> + Copy
{
    type Output = Offset<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Offset::new(self.x * rhs, self.y * rhs)
    }
}

impl<T> Div<T> for Offset<T> where T: Div<Output=T> + Copy
{
    type Output = Offset<T>;
    fn div(self, rhs: T) -> Self::Output {
        Offset::new(self.x / rhs, self.y / rhs)
    }
}

impl<T> Rem<T> for Offset<T> where T: Rem<Output=T> + Copy
{
    type Output = Offset<T>;
    fn rem(self, rhs: T) -> Self::Output {
        Offset::new(self.x % rhs, self.y % rhs)
    }
}

impl<T> From<(T, T)> for Offset<T>
{
    fn from(value: (T, T)) -> Self {
        Offset::new(value.0, value.1)
    }
}

impl Offset<f32> {
    pub fn infinite() -> Offset<f32> {
        lazy_static! {
            pub static ref INFINITE: Offset<f32> = Offset {
                x: f32::INFINITY,
                y: f32::INFINITY
            };
        }
        return *INFINITE;
    }

    pub fn unspecified() -> Offset<f32> {
        lazy_static! {
            pub static ref UNSPECIFIED: Offset<f32> = Offset {
                x: f32::NAN,
                y: f32::NAN,
            };
        }

        return *UNSPECIFIED;
    }

    pub fn get_distance(&self) -> f32 {
        let (x, y) = (self.x, self.y);
        (x * x + y * y).sqrt()
    }

    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
}

impl<T> Default for Offset<T>
    where
        T: Default,
{
    fn default() -> Self {
        Self::new(T::default(), T::default())
    }
}

impl<T> PartialEq for Offset<T>
    where
        T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl IntOffset {
    pub fn as_f32_offset(&self) -> Offset<f32> {
        Offset::new(self.x as f32, self.y as f32)
    }
}