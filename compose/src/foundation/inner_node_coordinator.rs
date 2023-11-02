use std::{cell::RefCell};
use std::rc::Weak;

use auto_delegate::Delegate;
use crate::foundation::layout_node::LayoutNode;

use super::{measurable::MultiChildrenMeasurePolicy, node_coordinator::NodeCoordinatorImpl};

#[derive(Delegate)]
pub(crate) struct InnerNodeCoordinator {
    #[to(Placeable, Measured, NodeCoordinatorTrait, MeasureScope)]
    pub(crate) node_coordinator_impl: NodeCoordinatorImpl,
    pub(crate) layout_node: Weak<RefCell<LayoutNode>>,
    pub(crate) measure_policy: MultiChildrenMeasurePolicy,
}