use std::{any::Any, cell::RefCell, rc::Rc};
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use crate::foundation::remember_observer::RememberObserverItem;

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
    Group {
        hash: u64,
        depth: usize,
        skipping: bool,
        slot_data: Rc<RefCell<Vec<SlotTableType>>>,
    },
    LayoutNodeType(Rc<RefCell<LayoutNode>>),
    CustomType(Rc<RefCell<dyn Any>>),
}

impl GroupKind {
    pub(crate) fn visit_layout_node(&self, mut visitor: &mut impl FnMut(&Rc<RefCell<LayoutNode>>)) {
        match self {
            GroupKind::Group { slot_data, .. } => {
                for slot_table_type in slot_data.borrow().iter() {
                    slot_table_type.data.visit_layout_node(visitor);
                }
            }
            GroupKind::LayoutNodeType(layout_node) => {
                visitor(layout_node);
            }
            _ => {}
        }
    }

    pub fn visit_remember_observer_item_mut(&self, mut visitor: impl FnMut(&mut RememberObserverItem)) {
        match self {
            GroupKind::Group { slot_data, .. } => {
                for slot_table_type in slot_data.borrow().iter() {
                    slot_table_type.data.visit_remember_observer_item_mut(&mut visitor);
                }
            }
            GroupKind::CustomType(obj) => {
                if let Some(remember_observer_item) = obj.borrow_mut().downcast_mut::<RememberObserverItem>() {
                    visitor(remember_observer_item);
                }
            }
            _ => {}
        }
    }
}

impl Debug for GroupKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GroupKind::Empty => {
                f.debug_struct("GroupKind::Empty").finish()
            }
            GroupKind::Group { hash, depth, skipping, slot_data } => {
                f.debug_struct("GroupKind::Group")
                    .field("hash", hash)
                    .field("depth", depth)
                    .field("skipping", skipping)
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
