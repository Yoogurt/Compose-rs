use std::any::Any;

use auto_delegate::delegate;

#[delegate]
pub trait IntrinsicMeasurable {
    fn get_parent_data(&self) -> Option<&dyn Any>;

    fn min_intrinsic_width(&self, height: usize) -> usize {
        0
    }
    fn max_intrinsic_width(&self, height: usize) -> usize {
        0
    }
    fn min_intrinsic_height(&self, width: usize) -> usize {
        0
    }
    fn max_intrinsic_height(&self, width: usize) -> usize {
        0
    }
}
