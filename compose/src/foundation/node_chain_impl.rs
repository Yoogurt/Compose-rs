use std::rc::Rc;
use std::cell::RefCell;

use super::inner_coodinator::InnerCoordinator;
use super::look_ahead_capable_placeable::LayoutNodeWrapperImpl;
use super::modifier::Modifier;
use super::node_chain::NodeChain;
use super::outer_coordinator::OuterCoordinator;

impl NodeChain {
    pub(crate) fn new() -> Self {
        NodeChain {
            modifier: Modifier,
            parent_data: None,
            measure_result: Default::default(),
            inner_placeable: Rc::new(RefCell::new(InnerCoordinator::new())),
            outer_measurable_placeable: OuterCoordinator::new(),
            outer_layout_node: Rc::new(RefCell::new(LayoutNodeWrapperImpl::new())),
        }
    }
}