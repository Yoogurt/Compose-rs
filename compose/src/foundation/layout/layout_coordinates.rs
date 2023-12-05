use std::rc::Rc;
use std::fmt::Debug;
use std::cell::RefCell;
use auto_delegate::delegate;
use crate::foundation::geometry::{IntOffset, IntSize};

#[delegate]
pub trait LayoutCoordinates: Debug {
    fn size(&self) -> IntSize;

    fn is_attached(&self) -> bool;

    fn get_parent_coordinates(&self) -> Option<Rc<RefCell<dyn LayoutCoordinates>>>;

    fn get_parent_layout_coordinates(&self) -> Option<Rc<RefCell<dyn LayoutCoordinates>>>;
}