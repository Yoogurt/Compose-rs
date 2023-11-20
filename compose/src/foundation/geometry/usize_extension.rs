pub(crate) trait MayBeOverflowAdd<T> {
    fn add_signed(self, sized: T) -> Self;
}

impl MayBeOverflowAdd<i32> for usize {
    fn add_signed(self, sized: i32) -> usize {
        match self.checked_add_signed(sized as isize) {
            Some(ret) => {
                ret
            }
            None => {
                if sized > 0 {
                    usize::MAX
                } else {
                    usize::MIN
                }
            }
        }
    }
}