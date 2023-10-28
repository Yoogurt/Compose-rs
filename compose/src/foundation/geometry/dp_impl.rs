use std::ops::{Add, Sub};
use crate::foundation::geometry::{Dp, IntoDp};

impl IntoDp for f32 {
    fn dp(self) -> Dp {
        Dp::new(self)
    }
}

impl From<Dp> for f32 {
    fn from(value: Dp) -> Self {
        value.value
    }
}

impl From<f32> for Dp {
    fn from(value: f32) -> Self {
        value.dp()
    }
}

impl Add for Dp {
    type Output = Dp;

    fn add(self, rhs: Self) -> Self::Output {
        (self.value + rhs.value).dp()
    }
}

impl Sub for Dp {
    type Output = Dp;

    fn sub(self, rhs: Self) -> Self::Output {
        (self.value - rhs.value).dp()
    }
}

impl Dp {
    pub const INFINITE: Dp = Self::new(f32::INFINITY);
    pub const ZERO: Dp = Self::new(0.0);
    pub const UNSPECIFIC: Dp = Self::new(f32::NAN);

    const fn new(value: f32) -> Self {
        Self {
            value
        }
    }

    pub fn is_infinite(&self) -> bool {
        self.value == f32::MAX
    }

    pub fn is_unspecific(&self) -> bool {
        self.value.is_nan()
    }
}