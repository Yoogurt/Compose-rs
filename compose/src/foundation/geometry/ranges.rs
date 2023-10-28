use std::ops::RangeInclusive;

pub trait CoerceIn where Self: Ord + Copy + Sized {
    fn coerce_in(self, range: RangeInclusive<Self>) -> Self {
        if self > *range.end() {
            *range.end()
        } else if self < *range.start() {
            *range.start()
        } else {
            self
        }
    }
}

pub trait CoerceAtLeast where Self: Ord + Copy + Sized {
    fn coerce_at_least(self, minimum_value: Self) -> Self {
        self.max(minimum_value)
    }
}

pub trait CoerceAtMost where Self: Ord + Copy + Sized {
    fn coerce_at_most(self, maximum_value: Self) -> Self {
        self.min(maximum_value)
    }
}

impl<T> CoerceIn for T where T: Ord + Copy + Sized {}

impl<T> CoerceAtLeast for T where T: Ord + Copy + Sized {}

impl<T> CoerceAtMost for T where T: Ord + Copy + Sized {}