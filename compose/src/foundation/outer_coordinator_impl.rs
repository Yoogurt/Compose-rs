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
}

impl Measurable for OuterCoordinator {
    fn measure(&mut self, constraint: &Constraint) -> &mut dyn Placeable {
        // self.remeasure(constraint);
        &mut self.layout_node_wrapper
    }
}