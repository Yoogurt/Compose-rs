use auto_delegate::delegate;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

#[delegate]
pub trait IntrinsicMeasurable {
    fn set_parent_data(&mut self, parent_data: Option<Rc<RefCell<dyn Any>>>);
    fn get_parent_data(&self) -> Option<Rc<RefCell<dyn Any>>>;
    fn get_parent_data_ref(&self) -> Option<&Rc<RefCell<dyn Any>>>;

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
