use std::hash::{Hash, Hasher};
use std::ops::{Add, Sub};

use crate::foundation::geometry::{Dp, IntoDp};

impl IntoDp for f32 {
    fn dp(self) -> Dp {
        Dp::new(self as f64)
    }
}

impl IntoDp for f64 {
    fn dp(self) -> Dp {
        Dp::new(self)
    }
}

impl IntoDp for i32 {
    fn dp(self) -> Dp {
        Dp::new(self as f64)
    }
}

impl IntoDp for usize {
    fn dp(self) -> Dp {
        Dp::new(self as f64)
    }
}

impl From<Dp> for f32 {
    fn from(value: Dp) -> Self {
        value.value as f32
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
    pub const INFINITE: Dp = Self::new(f64::INFINITY);
    pub const ZERO: Dp = Self::new(0.0);
    pub const UNSPECIFIC: Dp = Self::new(f64::NAN);

    const fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn is_infinite(&self) -> bool {
        self.value == f64::MAX
    }

    pub fn is_unspecific(&self) -> bool {
        self.value.is_nan()
    }
}

impl Default for Dp {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Hash for Dp {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i64(self.value as i64)
    }
}
