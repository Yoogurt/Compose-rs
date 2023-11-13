use std::{cell::RefCell, rc::Rc};
use std::any::Any;

use crate::foundation::slot_table::{SlotReader, SlotTable, SlotWriter};
use crate::foundation::slot_table_type::{GroupKindIndex, SlotTableType};

use super::{constraint::Constraints, layout_node::LayoutNode, slot_table_type::GroupKind};

#[derive(Debug)]
enum ChangeType {
    Changes,
    FixUp,
    InsertUpFixUp,
    DeferredChange,
}

struct Change {
    change: Box<dyn FnOnce()>,
    change_type: ChangeType,
    sequence: usize,
}

pub(crate) struct ComposerInner {
    pub(crate) hash: i64,
    pub(crate) depth: usize,
    pub(crate) node_expected: bool,
    pub(crate) sequence: usize,

    pub(crate) inserting: bool,
    pub(crate) layout_node_stack: Vec<Rc<RefCell<LayoutNode>>>,
    pub(crate) slot_table: SlotTable,
    pub(crate) root: Option<Rc<RefCell<LayoutNode>>>,

    fix_up: Vec<Change>,
    insert_up_fix_up: Vec<Change>,
    deferred_changes: Vec<Change>,

    changes: Vec<Change>,

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

    pub(crate) fn dispatch_layout_to_first_layout_node(&self, _constraint: &Constraints) {}

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
        self.start(
            hash,
            None,
            GroupKind::Group {
                hash,
                depth: self.depth,
            },
            None,
        );
    }

    pub(crate) fn create_node(&mut self, factory: Box<dyn FnOnce(Rc<RefCell<LayoutNode>>)>) -> Rc<RefCell<LayoutNode>> {
        self.validate_node_expected();

        let node = LayoutNode::new();
        {
            let node = node.clone();
            self.record_fix_up(Box::new(move || {
                factory(node);
            }));
        }
        let mut slot_table_mut = self.slot_table.slots.borrow_mut();
        let parent = self
            .writer
            .get_group_kind(GroupKindIndex::LayoutNode, &mut slot_table_mut);

        {
            let node = node.clone();
            match parent {
                None => {
                    let root = self.root.clone().unwrap();
                    drop(slot_table_mut);
                    if root.as_ptr() == node.as_ptr() {
                        dbg!("skipping attach root to itself");
                        return node;
                    }

                    self.record_insert_up_fix_up(Box::new(move || {
                        LayoutNode::adopt_child(&root, &node, true);
                    }));
                }
                Some(parent) => match parent {
                    GroupKind::LayoutNodeType(parent) => {
                        let parent = parent.clone();
                        drop(slot_table_mut);
                        self.record_insert_up_fix_up(Box::new(move || {
                            LayoutNode::adopt_child(&parent, &node, false);
                        }));
                    }
                    _ => {
                        panic!("parent with wrong type")
                    }
                },
            }
        }

        self.writer.begin_insert_layout_node(node.clone());

        node
    }

    pub(crate) fn use_node(&mut self) -> Rc<RefCell<LayoutNode>> {
        todo!()
    }

    pub(crate) fn record_fix_up(&mut self, fix_up: Box<dyn FnOnce()>) {
        self.fix_up.push(Change {
            change: fix_up,
            change_type: ChangeType::FixUp,
            sequence: self.sequence,
        });
        self.sequence += 1;
    }

    pub(crate) fn record_insert_up_fix_up(&mut self, insert_up_fix_up: Box<dyn FnOnce()>) {
        self.insert_up_fix_up.push(Change {
            change: insert_up_fix_up,
            change_type: ChangeType::InsertUpFixUp,
            sequence: self.sequence,
        });
        self.sequence += 1;
    }

    pub(crate) fn record_deferred_change(&mut self, deferred_change: Box<dyn FnOnce()>) {
        self.deferred_changes.push(Change {
            change: deferred_change,
            change_type: ChangeType::DeferredChange,
            sequence: self.sequence,
        });
        self.sequence += 1;
    }

    pub(crate) fn register_insert_up_fix_up(&mut self) {
        if let Some(insert_up_fix_up) = self.insert_up_fix_up.pop() {
            self.fix_up.push(insert_up_fix_up)
        }
    }

    pub(crate) fn apply_changes(&mut self) {
        let mut changes = Vec::<Change>::new();
        std::mem::swap(&mut self.changes, &mut changes);
        changes.into_iter().for_each(|change| {
            println!("apply change sequence: {} , type: {:?}", change.sequence, change.change_type);
            (change.change)();
        });
    }

    pub(crate) fn apply_deferred_changes(&mut self) {
        let mut deferred_changes = Vec::<Change>::new();
        std::mem::swap(&mut self.deferred_changes, &mut deferred_changes);
        deferred_changes.into_iter().for_each(|change| {
            println!("apply deferred change sequence: {} , type: {:?}", change.sequence, change.change_type);
            (change.change)();
        });
    }

    fn record_slot_editing_operation(&mut self, action: impl FnOnce() + 'static) {
        self.changes.push(Change {
            change: Box::new(action),
            change_type: ChangeType::Changes,
            sequence: self.sequence,
        });
    }

    fn record_insert(&mut self) {
        if self.fix_up.is_empty() {} else {
            let mut fix_ups: Vec<Change> = vec![];
            std::mem::swap(&mut fix_ups, &mut self.fix_up);

            self.record_slot_editing_operation(move || {
                fix_ups.into_iter().for_each(|change| {
                    println!("apply change sequence: {} , type: {:?}", change.sequence, change.change_type);
                    (change.change)();
                });
            });
        }
    }

    pub(crate) fn end_node(&mut self) {
        self.writer.end_insert_layout_node();
        self.register_insert_up_fix_up();

        self.record_insert();
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

    pub(crate) fn start(
        &mut self,
        key: i64,
        object_key: Option<Box<dyn Any>>,
        group_kind: GroupKind,
        data: Option<Box<dyn Any>>,
    ) {
        self.validate_node_not_expected();
        self.update_compound_hash_enter(key);

        if self.inserting {}
    }

    pub(crate) fn end(&mut self, key: i64) {
        self.update_compound_hash_exit(key);
    }

    fn next_slot(&mut self) -> Option<&dyn Any> {
        if self.inserting {
            None
        } else {
            todo!()
        }
    }

    pub(crate) fn cache<R, T>(&mut self, key: &R, calculation: impl FnOnce() -> T) -> &T where R: Sized + PartialEq<R> + 'static {
        let changed = self.changed(key);
        if changed {
            self.slot_table.op
        } else {
            let obj = self.next_slot().unwrap();
            obj.downcast_ref::<T>().unwrap().clone()
        }
    }

    pub(crate) fn changed<T>(&mut self, key: &T) -> bool where T: Sized + PartialEq<T> + 'static {
        if let Some(obj) = self.next_slot() {
            if let Some(obj_cast) = obj.downcast_ref::<T>() {
                return obj_cast == key;
            }
        }

        false
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
            sequence: 0,
            node_expected: false,
            inserting: false,
            layout_node_stack: vec![],
            slot_table: slot_table,
            root: None,
            fix_up: vec![],
            insert_up_fix_up: vec![],
            deferred_changes: vec![],
            changes: vec![],
            reader,
            writer,
        }
    }
}
