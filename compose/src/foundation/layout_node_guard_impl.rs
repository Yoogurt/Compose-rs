use std::ops::Deref;
use super::{Composer, LayoutNode, LayoutNodeGuard};

impl<'a> LayoutNodeGuard<'a> {
    pub(crate) fn new(node: &'a LayoutNode, composer: &'a Composer) -> LayoutNodeGuard<'a> {
        LayoutNodeGuard {
            node,
            composer,
            _data: Default::default()
        }
    }
}

impl Deref for LayoutNodeGuard<'_> {
    type Target = LayoutNode;

    fn deref(&self) -> &Self::Target {
        self.node
    }
}