use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::pin::Pin;
use crate::foundation::{Composer, LayoutNode, LayoutNodeGuard};

impl Composer {
    pub fn begin_group(&mut self, hash: i64) {
        *self.depth.get_mut() += 1;
        *self.hash.get_mut() ^= hash;
    }

    pub fn begin_node(&mut self) -> LayoutNodeGuard {
        let node = LayoutNode::default();
        self.slot_table.push(Box::new(node));

        let node = self.slot_table.last().unwrap();
        Self::validate_type(node, TypeId::of::<LayoutNode>());

        LayoutNodeGuard::new(node.downcast_ref().unwrap(), self)
    }

    pub fn end_group(&mut self, hash: i64) {
        *self.depth.get_mut() -= 1;
        *self.hash.get_mut() ^= hash;
    }

    pub fn validate_group(&self) -> bool {
        return *self.depth.borrow() == 0 && *self.hash.borrow() == 0;
    }

    pub fn skip_compose() {}

    pub fn skip_to_group() {}

    fn validate_type(target: impl AsRef<dyn Any>, expect: std::any::TypeId) {
        assert_eq!(target.as_ref().type_id(), expect);
    }
}

impl Default for Composer {
    fn default() -> Self {
        Composer {
            hash: RefCell::new(0),
            depth: RefCell::new(0),
            slot_table: Default::default(),
        }
    }
}