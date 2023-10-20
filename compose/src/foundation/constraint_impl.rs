use std::ops::{Range, RangeInclusive};
use crate::foundation::Constraint;

impl Constraint {
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

    pub const fn fixed(width : usize, height: usize) -> Constraint{
        Constraint {
            min_width: width,
            max_width: width,
            min_height: height,
            max_height: height
        }
    }
}