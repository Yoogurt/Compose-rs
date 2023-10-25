use std::{mem::MaybeUninit, cell::RefCell, rc::Rc};

use super::look_ahead_capable_placeable::{LayoutNodeWrapperImpl, LayoutNodeWrapper};

#[derive(Debug)]
pub(crate) struct OuterCoordinator {
    pub(crate) layout_node_wrapper: LayoutNodeWrapperImpl,
    pub(crate) layout_node: MaybeUninit<Rc<RefCell<dyn LayoutNodeWrapper>>>,
}