use std::ops::RangeInclusive;

use crate::foundation::geometry::{CoerceIn, IntOffset, IntSize};

#[derive(Debug, Clone, Copy, Default)]
pub struct Constraints {
    pub min_width: usize,
    pub max_width: usize,
    pub min_height: usize,
    pub max_height: usize,
}

impl Constraints {
    pub const INFINITE: usize = usize::MAX;

    pub const fn unbounded() -> Constraints {
        Constraints {
            min_width: 0,
            max_width: usize::MAX,
            min_height: 0,
            max_height: usize::MAX,
        }
    }

    pub const fn new(width: RangeInclusive<usize>, height: RangeInclusive<usize>) -> Constraints {
        Constraints {
            min_width: *width.start(),
            max_width: *width.end(),
            min_height: *height.start(),
            max_height: *height.end(),
        }
    }

    pub const fn fixed(width: usize, height: usize) -> Constraints {
        Constraints {
            min_width: width,
            max_width: width,
            min_height: height,
            max_height: height,
        }
    }

    pub const fn fixed_width(width: usize) -> Constraints {
        Constraints {
            min_width: width,
            max_width: width,
            min_height: 0,
            max_height: usize::MAX,
        }
    }

    pub const fn fixed_height(height: usize) -> Constraints {
        Constraints {
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

    pub fn constrain(&self, other_constraint: &Constraints) -> Constraints {
        Self::new(
            self.min_width.coerce_in(other_constraint.width_range())
                ..=self.max_width.coerce_in(other_constraint.width_range()),
            self.min_height.coerce_in(other_constraint.height_range())
                ..=self.max_height.coerce_in(other_constraint.height_range()),
        )
    }

    pub fn constrain_size(&self, size: IntSize) -> IntSize {
        (
            size.width.coerce_in(self.width_range()),
            size.height.coerce_in(self.height_range()),
        )
            .into()
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

    pub fn min_dimension(&self) -> (usize, usize) {
        (self.min_width, self.min_height)
    }

    pub fn max_dimension(&self) -> (usize, usize) {
        (self.max_width, self.max_height)
    }

    pub fn offset(&self, offset: IntOffset) -> Constraints {
        let min_width = Self::check_add_signed(self.min_width, offset.x);
        let max_width = Self::check_add_signed(self.max_width, offset.x);

        let min_height = Self::check_add_signed(self.min_height, offset.y);
        let max_height = Self::check_add_signed(self.max_height, offset.y);
        Constraints::new(min_width..=max_width, min_height..=max_height)
    }

    fn check_add_signed(origin: usize, offset: i32) -> usize {
        origin.checked_add_signed(offset as isize).unwrap_or(
            if offset > 0 {
                Constraints::INFINITE
            } else {
                0
            }
        )
    }
}

impl PartialEq for Constraints {
    fn eq(&self, other: &Self) -> bool {
        self.min_width == other.min_width
            && self.max_width == other.max_width
            && self.min_height == other.min_height
            && self.max_height == other.max_height
    }
}

impl From<(RangeInclusive<usize>, RangeInclusive<usize>)> for Constraints {
    fn from(value: (RangeInclusive<usize>, RangeInclusive<usize>)) -> Self {
        Self::new(value.0, value.1)
    }
}
