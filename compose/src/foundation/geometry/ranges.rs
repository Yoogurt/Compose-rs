use std::ops::RangeInclusive;

pub trait CoerceIn<T> where T: PartialOrd + Copy {
    fn coerce_in(self, range: RangeInclusive<T>) -> T;
}

impl<T> CoerceIn<T> for T  where T: PartialOrd + Copy{
    fn coerce_in(self, range: RangeInclusive<T>) -> T {
        if self > *range.end() {
            *range.end()
        } else if self < *range.start() {
            *range.start()
        } else {
            self
        }
    }
}