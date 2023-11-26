use crate::foundation::composer::ScopeUpdateScope;
use crate::foundation::recompose_scope_impl::RecomposeScopeImpl;
use std::{cell::RefCell, rc::Rc};
use std::any::Any;
use std::cell::{Ref, RefMut};
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use crate::foundation::application_appiler::ApplicationApplier;
use crate::foundation::compose_node_lifecycle_callback::ComposeNodeLifecycleCallback;
use crate::foundation::composition::Composition;
use crate::foundation::derived_state::DerivedStateObserver;
use crate::foundation::pending::Pending;
use crate::foundation::recompose_scope_impl::RecomposeScope;
use crate::foundation::remember_manager::{RememberEventDispatcher, RememberManager};

use crate::foundation::slot_table::{SlotTable, SlotReadWriter};
use crate::foundation::slot_table_type::{GroupKindIndex, SlotTableType};
use crate::foundation::snapshot_value::SnapShotValue;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;

use super::{constraint::Constraints, layout_node::LayoutNode, slot_table_type::GroupKind};

#[derive(Debug)]
pub(crate) enum ChangeType {
    Changes,
    FixUp,
    InsertUpFixUp,
    DeferredChange,
}

pub(crate) struct Change {
    pub(crate) change: Box<dyn FnOnce(&mut dyn RememberManager)>,
    pub(crate) change_type: ChangeType,
    // sequence: usize,
}

impl Debug for Change {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Change")
            .field("change_type", &self.change_type)
            .field("change", &(self.change.as_ref() as *const dyn FnOnce(_)))
            // .field("sequence", &self.sequence)
            .finish()
    }
}

pub(crate) struct ComposerImpl {
    pub(crate) hash: u64,
    pub(crate) depth: usize,
    pub(crate) node_expected: bool,
    pub(crate) sequence: usize,

    pub(crate) inserting: bool,
    pub(crate) root: Option<Rc<RefCell<LayoutNode>>>,
    pub(crate) layout_node_stack: Vec<Rc<RefCell<LayoutNode>>>,
    pub(crate) composition: Composition,

    pub(crate) slot_table: SlotTable,

    pub(crate) read_writer: SlotReadWriter,

    fix_up: Vec<Change>,
    insert_up_fix_up: Vec<Change>,
    deferred_changes: Vec<Change>,
    changes: Vec<Change>,
    pending_stack: Vec<Option<Pending>>,

    invalidate_stack: Vec<Rc<RefCell<RecomposeScopeImpl>>>,
}

impl Debug for ComposerImpl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Composer")
            .field("hash", &self.hash)
            .field("depth", &self.depth)
            .field("slot_table", &self.slot_table)
            .field("inserting", &self.inserting)
            .field("fix_up", &self.fix_up)
            .field("insert_up_fix_up", &self.insert_up_fix_up)
            .field("deferred_changes", &self.deferred_changes)
            .finish()
    }
}

impl ComposerImpl {
    const ROOT_KEY: u64 = 100;
    const NODE_KEY: u64 = 125;

    pub(crate) fn destroy(&mut self) {
        self.slot_table.slots.borrow_mut().clear();
        self.root = None;
        self.fix_up.clear();
        self.insert_up_fix_up.clear();
        self.deferred_changes.clear();
        self.invalidate_stack.clear();
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

    pub(crate) fn skipping(&self) -> bool {
        self.read_writer.slot.borrow().first().map(|group| match group.data {
            GroupKind::Group { skipping, .. } => {
                skipping
            }
            _ => {
                panic!()
            }
        }).unwrap_or(false)
    }

    pub(crate) fn skip_to_end(&self) {}

    pub(crate) fn start_root(&mut self) {
        {
            let writer = &mut self.read_writer;
            writer.slot_index_stack.clear();
            writer.current_slot_index = 0;
        }
        self.start_group(Self::ROOT_KEY);
    }

    pub(crate) fn end_root(&mut self) {
        self.end_group(Self::ROOT_KEY);
        {
            let writer = &mut self.read_writer;
            writer.slot_index_stack.clear();
            writer.current_slot_index = 0;
        }
    }

    pub(crate) fn start_node(&mut self) {
        self.node_expected = true
    }

    pub(crate) fn end_group(&mut self, hash: u64) {
        self.read_writer.exit_group();
        self.end(hash);
    }

    pub(crate) fn start_group(&mut self, hash: u64) {
        self.start(
            hash,
            None,
            GroupKind::Group {
                hash,
                depth: self.depth,
                skipping: false,
                slot_data: vec![].wrap_with_rc_refcell(),
            },
            None,
        );
    }

    pub(crate) fn create_node(&mut self, factory: impl FnOnce(Rc<RefCell<LayoutNode>>) + 'static) -> Rc<RefCell<LayoutNode>> {
        self.validate_node_expected();

        let node = LayoutNode::new();
        {
            let node = node.clone();
            self.record_fix_up(move |_| {
                factory(node);
            });
        }
        self.read_writer.begin_insert_layout_node(node.clone());

        let parent = self.read_writer.parent_layout_node();

        {
            let node = node.clone();
            match parent {
                None => {
                    let root = self.root.clone().unwrap();
                    if root.as_ptr() == node.as_ptr() {
                        dbg!("skipping attach root to itself");
                        return node;
                    }

                    self.record_insert_up_fix_up(move |_| {
                        LayoutNode::adopt_child(&root, &node, true);
                    });
                }
                Some(parent) => {
                    self.record_insert_up_fix_up(move |_| {
                        LayoutNode::adopt_child(&parent, &node, false);
                    });
                }
            }
        }

        node
    }

    pub(crate) fn use_node(&mut self) -> Rc<RefCell<LayoutNode>> {
        self.validate_node_expected();

        let node = self.read_writer.use_layout_node();
        self.read_writer.begin_use_layout_node(node.clone());

        let node_ref = node.clone();
        self.record_applier_operation(move |_| {
            node_ref.borrow_mut().on_reuse();
        });

        node
    }

    pub(crate) fn record_fix_up(&mut self, fix_up: impl FnOnce(&mut dyn RememberManager) + 'static) {
        self.fix_up.push(Change {
            change: Box::new(fix_up),
            change_type: ChangeType::FixUp,
            // sequence: self.sequence,
        });
        self.sequence += 1;
    }

    pub(crate) fn record_insert_up_fix_up(&mut self, insert_up_fix_up: impl FnOnce(&mut dyn RememberManager) + 'static) {
        self.insert_up_fix_up.push(Change {
            change: Box::new(insert_up_fix_up),
            change_type: ChangeType::InsertUpFixUp,
            // sequence: self.sequence,
        });
        self.sequence += 1;
    }

    pub(crate) fn record_deferred_change(&mut self, deferred_change: impl FnOnce(&mut dyn RememberManager) + 'static) {
        self.deferred_changes.push(Change {
            change: Box::new(deferred_change),
            change_type: ChangeType::DeferredChange,
            // sequence: self.sequence,
        });
        self.sequence += 1;
    }

    pub(crate) fn register_insert_up_fix_up(&mut self) {
        if let Some(insert_up_fix_up) = self.insert_up_fix_up.pop() {
            self.fix_up.push(insert_up_fix_up)
        }
    }

    pub(crate) fn apply_deferred_changes(&mut self) {
        let mut deferred_changes = Vec::<Change>::new();
        std::mem::swap(&mut self.deferred_changes, &mut deferred_changes);

        let mut remember_dispatcher = RememberEventDispatcher::new();
        deferred_changes.into_iter().for_each(|change| {
            (change.change)(&mut remember_dispatcher);
        });
    }

    fn record_applier_operation(&mut self, action: impl FnOnce(&mut dyn RememberManager) + 'static) {
        self.composition.record(action)
    }

    fn record_slot_editing_operation(&mut self, action: impl FnOnce(&mut dyn RememberManager) + 'static) {
        self.composition.record(action)
    }

    fn record_insert(&mut self) {
        if self.fix_up.is_empty() {} else {
            let mut fix_ups: Vec<Change> = vec![];
            std::mem::swap(&mut fix_ups, &mut self.fix_up);

            self.record_slot_editing_operation(move |remember_manager| {
                fix_ups.into_iter().for_each(|change| {
                    (change.change)(remember_manager);
                });
            });
        }
    }

    pub(crate) fn end_node(&mut self) {
        if self.inserting() {
            self.read_writer.end_insert_layout_node();
            self.register_insert_up_fix_up();
            self.record_insert();
        } else {
            self.read_writer.end_insert_layout_node();
        }
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
        if self.inserting {
            panic!("validate group inserting")
        }

        self.read_writer.validate();
    }

    pub(crate) fn debug_print(&self) {
        dbg!(self);
    }

    fn add_recompose_scope(&mut self) {
        if self.inserting() {
            let recompose_scope_impl = RecomposeScopeImpl::new().wrap_with_rc_refcell();
            self.invalidate_stack.push(recompose_scope_impl.clone());
            self.update_value(recompose_scope_impl.clone());
            recompose_scope_impl.borrow_mut().start(0);
        } else {
            self.read_writer.skip_slot();
            // perform recompose scope update
        }
    }

    pub(crate) fn start_restart_group(&mut self) {
        self.add_recompose_scope()
    }

    pub(crate) fn end_restart_group(&mut self) -> Option<Rc<RefCell<dyn ScopeUpdateScope>>> {
        self.invalidate_stack.pop().map(|scope| scope as Rc<RefCell<dyn ScopeUpdateScope>>)
    }

    pub(crate) fn recompose_scope(&self) -> Option<Rc<RefCell<dyn RecomposeScope>>> {
        self.invalidate_stack.last().map(|scope| scope.clone() as Rc<RefCell<dyn RecomposeScope>>)
    }

    fn update_compound_hash_enter(&mut self, hash: u64) {
        self.hash = self.hash.rotate_left(3);
        self.hash ^= hash;
        self.depth += 1;
    }

    fn update_compound_hash_exit(&mut self, hash: u64) {
        self.depth -= 1;
        self.hash ^= hash;
        self.hash = self.hash.rotate_right(3);
    }

    pub(crate) fn start(
        &mut self,
        key: u64,
        object_key: Option<Box<dyn Any>>,
        group_kind: GroupKind,
        data: Option<Box<dyn Any>>,
    ) {
        self.validate_node_not_expected();
        self.update_compound_hash_enter(key);

        if self.inserting() {
            self.read_writer.begin_empty();
            self.read_writer.begin_insert_group(self.hash, self.depth);
        } else {
            let group_ref = self.read_writer.slot.borrow();
            let last_slot_table_type = group_ref.get(self.read_writer.current_slot_index);
            let need_insert_group = last_slot_table_type.is_none();
            let need_replace_group = match last_slot_table_type {
                Some(slot_table_type) => {
                    match slot_table_type.data {
                        GroupKind::Group { hash, depth, .. } => {
                            hash != self.hash || depth != self.depth
                        }
                        _ => {
                            panic!("not a group")
                        }
                    }
                }

                None => {
                    false
                }
            };
            drop(group_ref);
            if need_insert_group {
                self.read_writer.begin_empty();
                self.read_writer.begin_insert_group(self.hash, self.depth);
                self.inserting = true;
            } else if need_replace_group {
                let slot_table_type = self.read_writer.replace_group(self.hash, self.depth);
                self.read_writer.begin_empty();
                self.inserting = true;
            } else {
                self.read_writer.skip_slot();
            }
        }

        self.read_writer.enter_group();
    }

    pub(crate) fn end(&mut self, key: u64) {
        self.update_compound_hash_exit(key);
        if self.inserting() {
            self.read_writer.end_empty();
            self.read_writer.end_insert_group(GroupKindIndex::Group);

            if !self.read_writer.in_empty() {
                self.inserting = false;
            }
        }

        while !self.read_writer.is_group_end() {
            let slot_to_remove = self.read_writer.pop_current_slot();

            slot_to_remove.data.visit_layout_node(&mut |layout_node| {
                // self.remember_dispatcher.deactivate(layout_node.clone())
            });

            slot_to_remove.data.visit_lifecycle_observer(&mut |item| {
                // self.remember_dispatcher.forgetting(item.clone())
            });
        }
    }

    fn next_slot(&mut self) -> Option<&dyn Any> {
        if self.inserting() {
            None
        } else {
            todo!()
        }
    }

    fn update_value(&mut self, value: Rc<RefCell<dyn Any>>) {
        if self.inserting() {
            self.read_writer.update(value);
        } else {
            todo!()
        }
    }

    pub(crate) fn cache<R, T>(&mut self, key: &R, calculation: impl FnOnce() -> T) -> SnapShotValue<T>
        where T: 'static, R: Sized + PartialEq<R> + 'static {
        let changed = self.changed(key);
        if changed {
            let value = calculation();
            let obj = value.wrap_with_rc_refcell();
            self.update_value(obj.clone());
            return SnapShotValue::new(obj);
        } else {
            todo!()
        }
    }

    pub(crate) fn changed<T>(&mut self, key: &T) -> bool where T: Sized + PartialEq<T> + 'static {
        if let Some(obj) = self.next_slot() {
            if let Some(obj_cast) = obj.downcast_ref::<T>() {
                return obj_cast != key;
            }
        }

        true
    }

    pub(crate) fn has_pending_changes(&self) -> bool {
        !self.changes.is_empty()
    }

    pub(crate) fn apply_changes(&mut self) {
        self.composition.apply_changes();
    }
}

impl Default for ComposerImpl {
    fn default() -> Self {
        let mut slot_table = SlotTable::default();
        let read_writer = slot_table.open_read_writer();

        Self {
            hash: 0,
            depth: 0,
            sequence: 0,
            node_expected: false,
            inserting: false,
            layout_node_stack: vec![],
            slot_table,
            root: None,
            fix_up: vec![],
            insert_up_fix_up: vec![],
            deferred_changes: vec![],
            changes: vec![],
            pending_stack: vec![],
            read_writer,
            invalidate_stack: vec![],
            composition: Composition::new(ApplicationApplier::new())
        }
    }
}

impl DerivedStateObserver for ComposerImpl {
    fn start(&mut self) {
        todo!()
    }

    fn done(&mut self) {
        todo!()
    }
}