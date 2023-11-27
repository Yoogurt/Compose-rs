use std::{any::Any, cell::RefCell, rc::Rc};
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use crate::foundation::modifier::NodeKind;
use crate::foundation::remember_observer::{RememberObserver, RememberObserverDelegate};
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;

use super::layout_node::LayoutNode;

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) enum GroupKindIndex {
    Empty = 0,
    Group = 1,
    LayoutNode = 2,
    LifecycleObserver = 3,
    Custom = 4,
}

pub(crate) enum GroupKind {
    Empty,
    Group {
        hash: u64,
        depth: usize,
        skipping: bool,
        slot_data: Rc<RefCell<Vec<SlotTableData>>>,
    },
    Node(Option<Rc<RefCell<LayoutNode>>>),

    LifecycleObserver(Rc<RefCell<dyn RememberObserver>>),
    CustomType(Rc<RefCell<dyn Any>>),
}

impl From<GroupKind> for Rc<RefCell<GroupKind>> {
    fn from(value: GroupKind) -> Self {
        value.wrap_with_rc_refcell()
    }
}

impl GroupKind {
    pub(crate) fn visit_layout_node(&self, mut visitor: &mut impl FnMut(&Rc<RefCell<LayoutNode>>)) {
        match self {
            GroupKind::Group { slot_data, .. } => {
                for slot_table_type in slot_data.borrow().iter() {
                    // slot_table_type.data.visit_layout_node(visitor);
                }
            }
            GroupKind::Node(layout_node) => {
                // visitor(layout_node);
            }
            _ => {}
        }
    }

    pub fn visit_lifecycle_observer(&self, visitor: &mut impl FnMut(&Rc<RefCell<dyn RememberObserver>>)) {
        match self {
            GroupKind::Group { slot_data, .. } => {
                for slot_table_type in slot_data.borrow().iter() {
                    // slot_table_type.data.visit_lifecycle_observer(visitor);
                }
            }
            GroupKind::LifecycleObserver(obj) => {
                visitor(obj);
            }
            _ => {}
        }
    }

    pub fn is_node(&self) -> bool {
        match self {
            GroupKind::Node { .. } => { true }
            _ => { false }
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
            GroupKind::Node(layout_node) => {
                let identify = layout_node.as_ref().unwrap().borrow().identify;
                f.debug_struct("GroupKind::LayoutNodeType")
                    .field("layout_node", &format!("LayoutNode({identify})"))
                    .finish()
            }
            GroupKind::LifecycleObserver(observer) => {
                f.debug_struct("GroupKind::LifecycleObserver")
                    .field("observer", &observer.as_ptr())
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
            GroupKind::Node(_) => GroupKindIndex::LayoutNode,
            GroupKind::LifecycleObserver(_) => GroupKindIndex::LifecycleObserver,
            GroupKind::CustomType(_) => GroupKindIndex::Custom,
        }
    }
}

pub(crate) type SlotTableData = Rc<RefCell<GroupKind>>;