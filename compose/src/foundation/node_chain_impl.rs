use crate::foundation::{NodeChain, Modifier, InnerCoordinator, LayoutNodeWrapperImpl,OuterCoordinator};
use std::rc::Rc;
use std::cell::RefCell;

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