use std::{rc::Rc, cell::RefCell};

use super::layout_node::LayoutNode;

pub struct LayoutNodeGuard {
    pub(crate) inner: Rc<RefCell<LayoutNode>>
}
