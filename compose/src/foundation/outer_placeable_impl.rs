use std::cell::RefCell;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use crate::foundation::geometry::{IntOffset, IntSize};
use super::{Constraint, MeasuredImpl, OuterCoordinator, LayoutNode, LayoutState, Measurable, Placeable, LayoutNodeWrapper, Measured, PlaceAction, PlaceableImpl, LayoutNodeWrapperImpl};

impl Deref for OuterCoordinator {
    type Target = dyn LayoutNodeWrapper;

    fn deref(&self) -> &Self::Target {
        &self.layout_node_wrapper
    }
}

impl DerefMut for OuterCoordinator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.layout_node_wrapper
    }
}

impl OuterCoordinator {
    pub(crate) fn new() -> OuterCoordinator {
        OuterCoordinator {
            layout_node_wrapper: LayoutNodeWrapperImpl::new(),
            layout_node: MaybeUninit::uninit(),
        }
    }

    pub(crate) fn attach(&mut self, layout_node: Rc<RefCell<dyn LayoutNodeWrapper>>) {
        self.layout_node = MaybeUninit::new(layout_node);
    }

    pub(crate) fn remeasure(&mut self,
                            constraint: &Constraint) -> bool {
        let mut previous_size: IntSize;
        let new_size = {
            let mut inner_layout_node = unsafe { self.layout_node.assume_init_ref().borrow_mut() };
             previous_size = inner_layout_node.get_measured_size();

            inner_layout_node.measure(constraint);
            inner_layout_node.get_measured_size()
        };
        let size_changed = previous_size != new_size
            || self.get_width() != new_size.width() || self.get_height() != new_size.height();

        self.set_measured_size(new_size);
        size_changed
    }
}

impl Measurable for OuterCoordinator {
    fn measure(&mut self, constraint: &Constraint) -> &mut dyn Placeable {
        self.remeasure(constraint);
        &mut self.layout_node_wrapper
    }
}