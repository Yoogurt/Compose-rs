#[derive(Debug, Clone, Copy, Default)]
pub struct Constraint {
    pub min_width: usize,
    pub max_width: usize,
    pub min_height: usize,
    pub max_height: usize,
}

use std::ops::RangeInclusive;
use crate::foundation::geometry::{CoerceIn, IntSize};

impl Constraint {
    pub const INFINITE: usize = usize::MAX;

    pub const fn unbounded() -> Constraint {
        Constraint {
            min_width: 0,
            max_width: usize::MAX,
            min_height: 0,
            max_height: usize::MAX,
        }
    }

    pub const fn new(width: RangeInclusive<usize>, height: RangeInclusive<usize>) -> Constraint {
        Constraint {
            min_width: *width.start(),
            max_width: *width.end(),
            min_height: *height.start(),
            max_height: *height.end(),
        }
    }

    pub const fn fixed(width: usize, height: usize) -> Constraint {
        Constraint {
            min_width: width,
            max_width: width,
            min_height: height,
            max_height: height,
        }
    }

    pub const fn fixed_width(width: usize) -> Constraint {
        Constraint {
            min_width: width,
            max_width: width,
            min_height: 0,
            max_height: usize::MAX,
        }
    }

    pub const fn fixed_height(height: usize) -> Constraint {
        Constraint {
            min_width: 0,
            max_width: usize::MAX,
            min_height: height,
            max_height: height,
        }
    }

    pub const fn has_bounded_width(&self) -> bool {
        self.max_width != usize::MAX
    }

    pub const fn has_bounded_height(&self) -> bool {
        self.max_height != usize::MAX
    }

    pub const fn has_fixed_width(&self) -> bool {
        self.min_width == self.max_width
    }

    pub const fn has_fixed_height(&self) -> bool {
        self.min_height == self.max_height
    }

    pub fn constrain(&self, other_constraint: &Constraint) -> Constraint {
        Self::new(self.min_width.coerce_in(other_constraint.width_range())..=self.max_width.coerce_in(other_constraint.width_range()),
                  self.min_height.coerce_in(other_constraint.height_range())..=self.max_height.coerce_in(other_constraint.height_range()))
    }

    pub fn constrain_size(&self, size: IntSize) -> IntSize {
        (size.width().coerce_in(self.width_range()), size.height().coerce_in(self.height_range())).into()
    }

    pub fn constrain_width(&self, width: usize) -> usize {
        width.coerce_in(self.width_range())
    }

    pub fn constrain_height(&self, height: usize) -> usize {
        height.coerce_in(self.height_range())
    }

    pub fn width_range(&self) -> RangeInclusive<usize> {
        self.min_width..=self.max_width
    }

    pub fn height_range(&self) -> RangeInclusive<usize> {
        self.min_height..=self.max_height
    }
}

impl PartialEq for Constraint {
    fn eq(&self, other: &Self) -> bool {
        self.min_width == other.min_width
            && self.max_width == other.max_width
            && self.min_height == other.min_height
            && self.max_height == other.max_height
    }
}

impl From<(RangeInclusive<usize>, RangeInclusive<usize>)> for Constraint {
    fn from(value: (RangeInclusive<usize>, RangeInclusive<usize>)) -> Self {
        Self::new(value.0, value.1)
    }
}