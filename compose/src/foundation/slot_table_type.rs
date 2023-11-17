use std::{any::Any, cell::RefCell, rc::Rc};
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

use super::layout_node::LayoutNode;

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) enum GroupKindIndex {
    Empty = 0,
    Group = 1,
    LayoutNode = 2,
    Custom = 3,
}

pub(crate) enum GroupKind {
    Empty,
    Group { hash: i64, depth: usize, slot_data: Rc<RefCell<Vec<SlotTableType>>> },
    LayoutNodeType(Rc<RefCell<LayoutNode>>),
    CustomType(Rc<RefCell<dyn Any>>),
}

impl Debug for GroupKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GroupKind::Empty => {
                f.debug_struct("GroupKind::Empty").finish()
            }
            GroupKind::Group { hash, depth, slot_data } => {
                f.debug_struct("GroupKind::Group")
                    .field("hash", hash)
                    .field("depth", depth)
                    .field("slot_data", slot_data)
                    .finish()
            }
            GroupKind::LayoutNodeType(layout_node) => {
                let identify = layout_node.borrow().identify;
                f.debug_struct("GroupKind::LayoutNodeType")
                    .field("layout_node", &format!("LayoutNode({identify})"))
                    .finish()
            }
            GroupKind::CustomType(obj) => {
                f.debug_struct("GroupKind::CustomType")
                    .field("custom_type", &(obj.as_ptr()))
                    .finish()
            }
        }
    }
}

impl GroupKind {
    pub(crate) fn index(&self) -> GroupKindIndex {
        match self {
            GroupKind::Empty => GroupKindIndex::Empty,
            GroupKind::Group { .. } => GroupKindIndex::Group,
            GroupKind::LayoutNodeType(_) => GroupKindIndex::LayoutNode,
            GroupKind::CustomType(_) => GroupKindIndex::Custom,
        }
    }
}

#[derive(Debug)]
pub(crate) struct SlotTableType {
    pub(crate) data: GroupKind,
}
