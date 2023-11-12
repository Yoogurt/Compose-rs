use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use auto_delegate::Delegate;

use crate::foundation::constraint::Constraints;
use crate::foundation::geometry::IntSize;
use crate::foundation::intrinsic_measurable::IntrinsicMeasurable;
use crate::foundation::measurable::Measurable;
use crate::foundation::placeable::Placeable;
use crate::foundation::placeable_impl::PlaceableImpl;
use crate::foundation::remeasurable::Remeasurable;

#[derive(Delegate, Debug)]
pub(crate) struct LookaheadPassDelegate {
    #[to(Placeable, Measured)]
    pub(crate) placeable_impl: PlaceableImpl,
}

impl LookaheadPassDelegate {
    pub(crate) fn new() -> Self {
        LookaheadPassDelegate {
            placeable_impl: PlaceableImpl::new(),
        }
    }
}

impl Remeasurable for LookaheadPassDelegate {
    fn remeasure(&mut self, _constraint: &Constraints) -> bool {
        todo!()
    }
}

impl Measurable for LookaheadPassDelegate {
    fn measure(&mut self, _constraint: &Constraints) -> (IntSize, Rc<RefCell<dyn Placeable>>) {
        todo!()
    }

    fn as_placeable(&mut self) -> Rc<RefCell<dyn Placeable>> {
        todo!()
    }

    fn as_measurable_mut(&mut self) -> &mut dyn Measurable {
        self
    }
}

impl IntrinsicMeasurable for LookaheadPassDelegate {
    fn set_parent_data(&mut self, _parent_data: Option<Rc<RefCell<dyn Any>>>) {
        todo!()
    }

    fn get_parent_data(&self) -> Option<Rc<RefCell<dyn Any>>> {
        todo!()
    }

    fn get_parent_data_ref(&self) -> Option<&Rc<RefCell<dyn Any>>> {
        todo!()
    }
}
