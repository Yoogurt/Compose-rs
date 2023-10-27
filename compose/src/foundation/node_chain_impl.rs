use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::mem::MaybeUninit;
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;

use super::inner_node_coodinator::InnerNodeCoordinator;
use super::look_ahead_capable_placeable::NodeCoordinatorImpl;
use super::modifier::Modifier;
use super::node_chain::NodeChain;
use super::outer_coordinator::OuterCoordinator;

impl NodeChain {
    pub(crate) fn new() -> Rc<RefCell<Self>> {
        let inner_node_coordinator = InnerNodeCoordinator::new().wrap_with_rc_refcell();

        let result = NodeChain {
            modifier: Modifier,
            parent_data: None,
            measure_result: Default::default(),
            inner_coordinator: inner_node_coordinator.clone(),
            outer_coordinator: inner_node_coordinator,

            layout_node: MaybeUninit::uninit(),
        };

        let inner_coodinator = result.inner_coordinator.clone();
        result.wrap_with_rc_refcell()
    }

    pub(crate) fn attach(&mut self, layout_node: Weak<RefCell<LayoutNode>>) {
        self.layout_node = MaybeUninit::new(layout_node.clone());
        self.inner_coordinator.borrow_mut().attach(layout_node);
    }
}