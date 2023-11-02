use std::cell::RefCell;
use std::rc::{Rc, Weak};
use auto_delegate::Delegate;
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::node_coordinator::NodeCoordinatorImpl;
use crate::foundation::modifier::Node;

#[derive(Debug, Delegate)]
pub(crate) struct LayoutModifierNodeCoordinator {
    pub(crate) layout_node: Weak<RefCell<LayoutNode>>,
    #[to(Placeable, Measured, NodeCoordinatorTrait, MeasureScope, PlaceablePlaceAt)]
    pub(crate) node_coordinator_impl: NodeCoordinatorImpl,
    pub(crate) layout_modifier_node: Rc<RefCell<dyn Node>>,
}