use std::cell::RefCell;
use std::rc::{Rc, Weak};
use auto_delegate::Delegate;
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::layout_modifier_node::LayoutModifierNode;
use crate::foundation::look_ahead_capable_placeable::NodeCoordinatorImpl;
use crate::foundation::modifier::Node;

#[derive(Debug, Delegate)]
pub(crate) struct LayoutModifierNodeCoordinator {
    pub(crate) layout_node: Weak<RefCell<LayoutNode>>,
    // pub(crate) measure_node: &'a mut dyn LayoutModifierNode,
    #[to(Placeable, Measured)]
    pub(crate) node_coordinator_impl: NodeCoordinatorImpl,
    pub(crate) layout_modifier_node: Rc<RefCell<dyn Node>>,
}