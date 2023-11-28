use std::{any::Any, cell::RefCell, rc::Rc};
use std::fmt::{Debug, Formatter, Write};
use std::hash::Hash;
use crate::foundation::modifier::NodeKind;
use crate::foundation::remember_observer::{RememberObserver, RememberObserverDelegate};
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;

use super::layout_node::LayoutNode;

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) enum GroupKindIndex {
    // Empty = 0,
    Hash = 0,
    Group = 1,
    LayoutNode = 2,
    LifecycleObserver = 3,
    Custom = 4,
}

pub(crate) enum GroupKind {
    // Empty,
    Hash(u64),
    Group {
        key: u64,
        depth: usize,
        skipping: bool,
        slot_data: Rc<RefCell<Vec<Slot>>>,
    },

    Node { node: Option<Rc<RefCell<LayoutNode>>>, slot_data: Rc<RefCell<Vec<Slot>>> },
    LifecycleObserver(Rc<RefCell<dyn RememberObserver>>),

    CustomType(Rc<RefCell<dyn Any>>),
}

impl GroupKind {
    pub fn Node() -> GroupKind {
        GroupKind::Node { node: None, slot_data: vec![].wrap_with_rc_refcell() }
    }
    pub fn update_node(&mut self, layout_node: Rc<RefCell<LayoutNode>>) {
        match self {
            GroupKind::Node { node, .. } => {
                *node = Some(layout_node)
            }
            _ => {
                panic!("update node from wrong group kind")
            }
        }
    }
    pub fn node(&self) -> Rc<RefCell<LayoutNode>> {
        match self {
            GroupKind::Node { node, .. } => {
                node.as_ref().unwrap().clone()
            }
            _ => {
                panic!("update node from wrong group kind")
            }
        }
    }
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
            GroupKind::Node { node, .. } => {
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

    pub fn is_group(&self) -> bool {
        match self {
            GroupKind::Group { .. } => { true }
            _ => { false }
        }
    }
}

impl Debug for GroupKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GroupKind::Hash(key) => {
                f.write_str(&format!("GroupKind::Hash({})", key))
            }
            GroupKind::Group { key: hash, depth, skipping, slot_data } => {
                f.debug_struct("GroupKind::Group")
                    .field("hash", hash)
                    .field("depth", depth)
                    .field("skipping", skipping)
                    .field("slot_data", slot_data)
                    .finish()
            }
            GroupKind::Node { node, slot_data } => {
                match node.as_ref() {
                    Some(node) => {
                        let identify = node.borrow().identify;
                        f.debug_struct("GroupKind::Node")
                            .field("node", &format!("LayoutNode({identify})"))
                            .field("slot_data", slot_data)
                            .finish()
                    }
                    _ => {
                        f.debug_struct("GroupKind::Node").field("node", &"(Not Attach)").finish()
                    }
                }
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
            GroupKind::Hash(..) => { GroupKindIndex::Hash }
            GroupKind::Group { .. } => GroupKindIndex::Group,
            GroupKind::Node { .. } => GroupKindIndex::LayoutNode,
            GroupKind::LifecycleObserver(_) => GroupKindIndex::LifecycleObserver,
            GroupKind::CustomType(_) => GroupKindIndex::Custom,
        }
    }
}

pub(crate) type Slot = Rc<RefCell<GroupKind>>;