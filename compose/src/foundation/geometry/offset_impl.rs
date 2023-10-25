
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use lazy_static::lazy_static;
use super::{Offset, U64ConverterSigned, U64ConverterUnsigned};

impl U64ConverterUnsigned for i32 {
    fn as_u64(self) -> u64 {
        self as u64
    }

    fn from_u64(data: u64) -> Self {
        data as i32
    }
}

impl U64ConverterSigned for i32 {}

impl U64ConverterUnsigned for f32 {
    fn as_u64(self) -> u64 {
        self as u64
    }

    fn from_u64(data: u64) -> Self {
        data as f32
    }
}

impl U64ConverterSigned for f32 {}

impl<T> Offset<T> where T: U64ConverterUnsigned {
    #[inline]
    fn packed_value(x: T, y: T) -> u64 {
        let lhs = x.as_u64();
        let rhs = y.as_u64();
        lhs | (rhs << 32)
    }

    pub fn new(x: T, y: T) -> Offset<T> {
        Offset {
            packed_value: Self::packed_value(x, y),
            _data: Default::default(),
        }
    }

    pub fn zero() -> Offset<T> {
        Offset {
            packed_value: Self::packed_value(T::from_u64(0), T::from_u64(0)),
            _data: Default::default(),
        }
    }

    pub fn x(&self) -> T {
        T::from_u64(self.packed_value & 0xffffffff)
    }

    pub fn y(&self) -> T {
        T::from_u64((self.packed_value & 0xffffffff00000000) >> 32)
    }
}

impl<T> Neg for Offset<T> where T: U64ConverterSigned {
    type Output = Offset<T>;
    fn neg(self) -> Self::Output {
        Offset::new(-self.x(), -self.y())
    }
}

impl<T> Sub for Offset<T> where T: U64ConverterUnsigned {
    type Output = Offset<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Offset::new(self.x() - rhs.x(), self.y() - rhs.y())
    }
}

impl<T> Add for Offset<T> where T: U64ConverterUnsigned {
    type Output = Offset<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Offset::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

impl<T> Mul<T> for Offset<T> where T: U64ConverterUnsigned {
    type Output = Offset<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Offset::new(self.x() * rhs, self.y() * rhs)
    }
}

impl<T> Div<T> for Offset<T> where T: U64ConverterUnsigned {
    type Output = Offset<T>;
    fn div(self, rhs: T) -> Self::Output {
        Offset::new(self.x() / rhs, self.y() / rhs)
    }
}

impl<T> Rem<T> for Offset<T> where T: U64ConverterSigned {
    type Output = Offset<T>;
    fn rem(self, rhs: T) -> Self::Output {
        Offset::new(self.x() % rhs, self.y() % rhs)
    }
}

impl<T> From<(T, T)> for Offset<T> where T: U64ConverterUnsigned {
    fn from(value: (T, T)) -> Self {
        Offset::new(value.0, value.1)
    }
}

impl Offset<f32> {
    pub fn infinite() -> Offset<f32> {
        lazy_static! {
            pub static ref INFINITE : Offset<f32> = Offset {
                packed_value: Offset::<f32>::packed_value(f32::INFINITY, f32::INFINITY),
                _data: Default::default()
            };
        }
        return *INFINITE;
    }

    pub fn unspecified() -> Offset<f32> {
        lazy_static! {
            pub static ref UNSPECIFIED : Offset<f32> = Offset {
                packed_value: Offset::<f32>::packed_value(f32::NAN, f32::NAN),
                _data: Default::default()
            };
        }

        return *UNSPECIFIED;
    }

    pub fn get_distance(&self) -> f32 {
        let (x, y) = (self.x(), self.y());
        (x * x + y * y).sqrt()
    }

    pub fn is_finite(&self) -> bool {
        self.x().is_finite() && self.y().is_finite()
    }
}

#[test]
fn test_offset() {
    let mut offset = IntOffset::new(1, 2);
    dbg!(offset);
    dbg!(offset.x());
    dbg!(offset.y());
    dbg!(Offset::<f32>::infinite());
    dbg!(Offset::<f32>::unspecified());
}