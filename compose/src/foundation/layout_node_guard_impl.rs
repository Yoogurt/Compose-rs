#![allow(warnings)]
use std::cell::{Ref, RefMut, RefCell};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use super::layout_node::LayoutNode;
use super::layout_node_guard::LayoutNodeGuard;

impl LayoutNodeGuard {
    pub(crate) fn new(layout_node: Rc<RefCell<LayoutNode>>) -> LayoutNodeGuard {
        LayoutNodeGuard {
            inner: layout_node
        }
    }

    pub(crate) fn borrow(&self) -> Ref<'_, LayoutNode> {
        return self.inner.borrow();
    }

    pub(crate) fn borrow_mut(&self) -> RefMut<'_, LayoutNode> {
        return self.inner.borrow_mut();
    }
}