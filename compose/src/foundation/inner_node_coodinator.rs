use std::{rc::Rc, cell::RefCell, mem::MaybeUninit};
use std::rc::Weak;

use auto_delegate::Delegate;
use crate::foundation::layout_node::LayoutNode;

use super::{measurable::MultiChildrenMeasurePolicy, layout_node::LayoutNodeLayoutDelegate, look_ahead_capable_placeable::NodeCoordinatorImpl};

#[derive(Debug, Delegate)]
pub(crate) struct InnerNodeCoordinator {
    #[to(Placeable, Measured)]
    pub(crate) node_coordinator_impl: NodeCoordinatorImpl,
    pub(crate) layout_node: Weak<RefCell<LayoutNode>>,
    pub(crate) measure_policy: MultiChildrenMeasurePolicy,
}