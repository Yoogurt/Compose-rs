use std::{cell::RefCell, rc::Rc, any::Any};
use std::hash::{Hash, Hasher};

use super::{layout_node::LayoutNode, slot_table::SlotTable};

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) enum GroupKindIndex {
    Group = 1,
    LayoutNode = 2,
    Custom = 3,
}

#[derive(Debug)]
pub(crate) enum GroupKind {
    Group {
        hash: i64,
        depth: usize,
    },
    LayoutNodeType(Rc<RefCell<LayoutNode>>),
    CustomType(Box<dyn Any>),
}

impl GroupKind {
    pub(crate) fn index(&self) -> GroupKindIndex {
        match self {
            GroupKind::Group { .. } => {
                GroupKindIndex::Group
            }
            GroupKind::LayoutNodeType(_) => {
                GroupKindIndex::LayoutNode
            }
            GroupKind::CustomType(_) => {
                GroupKindIndex::Custom
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct SlotTableType {
    pub(crate) parent: usize,
    pub(crate) data: GroupKind,
}