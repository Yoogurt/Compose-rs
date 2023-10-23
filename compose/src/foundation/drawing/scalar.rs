pub struct Scalar {
    sx: f32,
    sy: f32,
}

impl Scalar {
    pub const ZERO :Self = Self::new(0.0,0.0);

    pub const fn new(sx: f32, sy: f32) -> Self {
        Scalar {
            sx,
            sy
        }
    }

    pub const fn sx(sx: f32) -> Self {
        Scalar {
            sx,
            sy: 1.0
        }
    }

    pub const fn sy(sy: f32) -> Self {
        Scalar {
            sx: 1.0,
            sy
        }
    }
}

impl From<(f32, f32)> for Scalar {
    fn from(value: (f32, f32)) -> Self {
        Self::new(value.0,value.1)
    }
}