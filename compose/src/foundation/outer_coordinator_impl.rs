use std::cell::RefCell;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use super::constraint::Constraint;
use super::layout_result::Placeable;
use super::look_ahead_capable_placeable::{NodeCoordinator, NodeCoordinatorImpl};
use super::measurable::Measurable;
use super::outer_coordinator::OuterCoordinator;

impl OuterCoordinator {
    pub(crate) fn new() -> OuterCoordinator {
        OuterCoordinator {
            node_coordinator_impl: NodeCoordinatorImpl::new(),
            layout_node: MaybeUninit::uninit(),
        }
    }

    pub(crate) fn attach(&mut self, layout_node: Rc<RefCell<dyn NodeCoordinator>>) {
        self.layout_node = MaybeUninit::new(layout_node);
    }
}

impl Measurable for OuterCoordinator {
    fn measure(&mut self, constraint: &Constraint) -> &mut dyn Placeable {
        self.node_coordinator_impl.perform_measure(constraint,| self_|{
            self_
        })
    }
}