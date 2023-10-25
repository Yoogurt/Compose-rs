use std::{rc::Rc, cell::RefCell, mem::MaybeUninit};

use super::{measurable::MultiChildrenMeasurePolicy, layout_node::LayoutNodeLayoutDelegate, look_ahead_capable_placeable::LayoutNodeWrapperImpl};

#[derive(Debug)]
pub(crate) struct InnerCoordinator {
    pub(crate) layout_node_wrapper_impl: LayoutNodeWrapperImpl,
    pub(crate) layout_node_layout_delegate: MaybeUninit<Rc<RefCell<LayoutNodeLayoutDelegate>>>,
    pub(crate) measure_policy: MultiChildrenMeasurePolicy,
}