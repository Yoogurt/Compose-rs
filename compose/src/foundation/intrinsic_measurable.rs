use std::any::Any;
use auto_delegate::delegate;

#[delegate]
pub trait IntrinsicMeasurable {
    fn set_parent_data(&mut self, parent_data: Option<Box<dyn Any>>);
    fn get_parent_data(&self) -> Option<&Box<dyn Any>>;
    fn get_parent_data_mut(&mut self) -> Option<&mut Box<dyn Any>>;

    fn min_intrinsic_width(&self, height: usize) -> usize { 0 }
    fn max_intrinsic_width(&self, height: usize) -> usize { 0 }
    fn min_intrinsic_height(&self, width: usize) -> usize { 0 }
    fn max_intrinsic_height(&self, width: usize) -> usize { 0 }
}