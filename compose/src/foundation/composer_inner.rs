use std::{rc::Rc, cell::RefCell};
use std::any::Any;
use crate::foundation::slot_table::{SlotTable, SlotReader, SlotWriter};
use crate::foundation::slot_table_type::GroupKindIndex;

use super::{constraint::Constraint, slot_table_type::GroupKind, layout_node::LayoutNode};

pub(crate) struct ComposerInner {
    pub(crate) hash: i64,
    pub(crate) depth: usize,
    pub(crate) node_expected: bool,

    pub(crate) inserting: bool,
    pub(crate) layout_node_stack: Vec<Rc<RefCell<LayoutNode>>>,
    pub(crate) slot_table: SlotTable,
    pub(crate) root: Option<Rc<RefCell<LayoutNode>>>,

    pub(crate) fix_up: Vec<Box<dyn FnOnce()>>,
    pub(crate) insert_up_fix_up: Vec<Box<dyn FnOnce()>>,
    pub(crate) deferred_changes: Vec<Box<dyn FnOnce()>>,

    pub(crate) reader: SlotReader,
    pub(crate) writer: SlotWriter,
}

impl ComposerInner {
    const ROOT_KEY: i64 = 100;
    const NODE_KEY: i64 = 125;

    pub(crate) fn destroy(&mut self) {
        self.slot_table.slots.borrow_mut().clear();
        self.root = None;
        self.fix_up.clear();
        self.insert_up_fix_up.clear();
        self.deferred_changes.clear();
    }

    pub(crate) fn dispatch_layout_to_first_layout_node(&self, _constraint: &Constraint) {
        // for slot_table_type in self.slot_table.slots.borrow().deref() {
        //     match slot_table_type.data {
        //         GroupKind::LayoutNodeType(_layout_node) => {
        //             // let measure_result = layout_node.borrow_mut().measure(constraint);
        //             // layout_node.borrow_mut().handle_measured_result(measure_result);
        //             return;
        //         }
        //         _ => {}
        //     }
        // }
    }

    pub(crate) fn attach_root_layout_node(&mut self, root: Rc<RefCell<LayoutNode>>) -> bool {
        if self.root.is_some() {
            return false;
        }

        self.root = Some(root);
        true
    }

    pub(crate) fn detach_root_layout_node(&mut self) {
        self.root = None;
    }

    pub(crate) fn inserting(&self) -> bool {
        self.inserting
    }

    pub(crate) fn start_root(&mut self) {
        self.inserting = true;
        self.start_group(Self::ROOT_KEY)
    }

    pub(crate) fn end_root(&mut self) {
        self.end_group(Self::ROOT_KEY);
        self.inserting = false;
    }

    pub(crate) fn start_node(&mut self) {
        self.node_expected = true
    }

    pub(crate) fn end_group(&mut self, hash: i64) {
        self.end(hash)
    }

    pub(crate) fn start_group(&mut self, hash: i64) {
        self.start(hash, None, GroupKind::Group {
            hash,
            depth: self.depth,
        }, None);
    }

    pub(crate) fn create_node(&mut self) -> Rc<RefCell<LayoutNode>> {
        self.validate_node_expected();

        let node = LayoutNode::new();
        self.writer.begin_insert_layout_node(node.clone());
        node
    }

    pub(crate) fn use_node(&mut self) -> Rc<RefCell<LayoutNode>> {
        todo!()
    }

    pub(crate) fn record_fix_up(&mut self, fix_up: Box<dyn FnOnce()>) {
        self.fix_up.push(fix_up)
    }

    pub(crate) fn record_insert_up_fix_up(&mut self, insert_up_fix_up: Box<dyn FnOnce()>) {
        self.insert_up_fix_up.push(insert_up_fix_up)
    }

    pub(crate) fn record_deferred_change(&mut self, deferred_change: Box<dyn FnOnce()>) {
        self.deferred_changes.push(deferred_change)
    }

    pub(crate) fn register_insert_up_fix_up(&mut self) {
        self.fix_up.push(self.insert_up_fix_up.pop().unwrap())
    }

    pub(crate) fn apply_changes(&mut self) {
        let mut fix_up = Vec::<Box<dyn FnOnce()>>::new();
        std::mem::swap(&mut self.fix_up, &mut fix_up);
        fix_up.into_iter().rev().for_each(|change| {
            change();
        });
    }

    pub(crate) fn apply_deferred_changes(&mut self) {
        let mut deferred_changes = Vec::<Box<dyn FnOnce()>>::new();
        std::mem::swap(&mut self.deferred_changes, &mut deferred_changes);
        deferred_changes.into_iter().for_each(|change| {
            change();
        });
    }

    pub(crate) fn end_node(&mut self) {
        let mut slot_table_mut = self.slot_table.slots.borrow_mut();
        let current_ref = self.writer.get_group_kind(GroupKindIndex::LayoutNode, &mut slot_table_mut).expect("unbalance create node pair");
        let current = match current_ref {
            GroupKind::LayoutNodeType(node) => {
                node.clone()
            }
            _ => {
                panic!("current node with wrong type")
            }
        };

        self.writer.end_insert_layout_node();

        let parent = self.writer.get_group_kind(GroupKindIndex::LayoutNode, &mut slot_table_mut);

        match parent {
            None => {
                let root = self.root.clone().unwrap();
                drop(slot_table_mut);
                if root.as_ptr() == current.as_ptr() {
                    dbg!("skipping attach root to itself");
                    return;
                }

                self.record_insert_up_fix_up(Box::new(move || {
                    LayoutNode::adopt_child(&root, &current, true);
                }));
            }
            Some(parent) => {
                match parent {
                    GroupKind::LayoutNodeType(parent) => {
                        let parent = parent.clone();
                        drop(slot_table_mut);
                        self.record_insert_up_fix_up(Box::new(move || {
                            LayoutNode::adopt_child(&parent, &current, false);
                        }));
                    }
                    _ => {
                        panic!("parent with wrong type")
                    }
                }
            }
        }

        self.register_insert_up_fix_up();
    }

    pub(crate) fn validate_node_expected(&mut self) {
        if !self.node_expected {
            panic!("A call to create_node(), emit_node() or use_node() expected was not expected")
        }
        self.node_expected = false
    }

    pub(crate) fn validate_node_not_expected(&mut self) {
        if self.node_expected {
            panic!("A call to create_node(), emit_node() or use_node() expected was expected")
        }
    }

    pub(crate) fn validate_group(&self) {
        if self.depth != 0 || self.hash != 0 {
            panic!("validate group fail")
        }
    }

    fn update_compound_hash_enter(&mut self, hash: i64) {
        self.hash = self.hash.rotate_left(3);
        self.hash ^= hash;
        self.depth += 1;
    }

    fn update_compound_hash_exit(&mut self, hash: i64) {
        self.hash ^= hash;
        self.hash = self.hash.rotate_right(3);
        self.depth -= 1;
    }

    pub(crate) fn start(&mut self, key: i64, object_key: Option<Box<dyn Any>>, group_kind: GroupKind, data: Option<Box<dyn Any>>) {
        self.validate_node_not_expected();
        self.update_compound_hash_enter(key);

        if self.inserting {}
    }

    pub(crate) fn end(&mut self, key: i64) {
        self.update_compound_hash_exit(key);
    }
}

impl Default for ComposerInner {
    fn default() -> Self {
        let mut slot_table = SlotTable::default();
        let reader = slot_table.open_reader();
        let writer = slot_table.open_writer();

        Self {
            hash: 0,
            depth: 0,
            node_expected: false,
            inserting: false,
            layout_node_stack: vec![],
            slot_table: slot_table,
            root: None,
            fix_up: vec![],
            insert_up_fix_up: vec![],
            deferred_changes: vec![],
            reader,
            writer,
        }
    }
}