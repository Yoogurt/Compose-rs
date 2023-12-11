use crate::foundation::geometry::IntOffset;
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use crate::foundation::layout::layout_coordinates::LayoutCoordinates;

pub(crate) struct HitPathTracker {
    root_coordinates: Rc<RefCell<dyn LayoutCoordinates>>
}

impl HitPathTracker {
    pub(crate) fn new(root_coordinates: Rc<RefCell<dyn LayoutCoordinates>>) -> HitPathTracker {
        HitPathTracker {
            root_coordinates
        }
    }

    // pub(crate) fn get_hit_path(&self, point: IntOffset) -> Vec<Rc<RefCell<dyn LayoutCoordinates>>> {
    //     let mut result = vec![];
    //     let mut current = self.root_coordinates.clone();
    //     while current.borrow().is_attached() {
    //         result.push(current.clone());
    //         current = current.borrow().get_parent_layout_coordinates().unwrap();
    //     }
    //     result
    // }
}