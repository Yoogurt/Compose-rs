use crate::foundation::composer::ScopeUpdateScope;
use crate::foundation::recompose_scope_impl::RecomposeScopeImpl;
use std::{cell::RefCell, rc::Rc};
use std::alloc::Layout;
use std::any::Any;
use std::cell::{Ref, RefMut};
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use crate::foundation::application_applier::ApplicationApplier;
use crate::foundation::applier::Applier;
use crate::foundation::compose_node_lifecycle_callback::ComposeNodeLifecycleCallback;
use crate::foundation::composition::Composition;
use crate::foundation::derived_state::DerivedStateObserver;
use crate::foundation::recompose_scope_impl::RecomposeScope;
use crate::foundation::remember_manager::{RememberEventDispatcher, RememberManager};

use crate::foundation::slot_table::{SlotTable, SlotReadWriter};
use crate::foundation::slot_table_type::{GroupKindIndex, Slot};
use crate::foundation::snapshot_value::SnapShotValue;
use crate::foundation::ui_applier::UiApplier;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;

use super::{constraint::Constraints, layout_node::LayoutNode, slot_table_type::GroupKind};

#[derive(Debug)]
pub(crate) enum ChangeType {
    Changes,
    FixUp,
    InsertUpFixUp,
    DeferredChange,
}

pub(crate) type ApplierInType = Rc<RefCell<LayoutNode>>;
pub(crate) type ChangeFn = dyn FnOnce(&mut dyn Applier<ApplierInType>, &mut dyn RememberManager);

pub(crate) struct Change {
    pub(crate) change: Box<ChangeFn>,
    pub(crate) change_type: ChangeType,
    // sequence: usize,
}

impl Change {
    pub fn new(block: impl FnOnce(&mut dyn Applier<ApplierInType>, &mut dyn RememberManager) + 'static) -> Self {
        Self {
            change: Box::new(block),
            change_type: ChangeType::Changes,
        }
    }
}

impl Debug for Change {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Change")
            .field("change_type", &self.change_type)
            // .field("change", &(self.change.as_ref() as *const dyn FnOnce(_)))
            // .field("sequence", &self.sequence)
            .finish()
    }
}

struct Pending {}

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
    changes: Vec<Change>,
    pending_stack: Vec<Option<Pending>>,

    invalidate_stack: Vec<Rc<RefCell<RecomposeScopeImpl>>>,

    previous_remove: i32,
    previous_count: usize,

    node_index: usize,
    node_index_stack: Vec<usize>,

    pending: Option<Pending>,
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
            .finish()
    }
}

pub(crate) const ROOT_KEY: u64 = 100;
pub(crate) const NODE_KEY: u64 = 125;

impl ComposerImpl {
    pub(crate) fn destroy(&mut self) {
        self.slot_table.slots.borrow_mut().clear();
        self.root = None;
        self.fix_up.clear();
        self.insert_up_fix_up.clear();
        self.invalidate_stack.clear();
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

    pub(crate) fn skipping(&self) -> bool {
        self.read_writer.skipping()
    }

    pub(crate) fn apply<V: 'static>(&mut self, value: V, block: impl FnOnce(&mut LayoutNode, V) + 'static) {
        let operation = move |applier: &mut dyn Applier<Rc<RefCell<LayoutNode>>>, _: &mut dyn RememberManager| {
            let mut layout_node = applier.get_current().borrow_mut();
            block(layout_node.deref_mut(), value);
        };

        if self.inserting {
            self.record_fix_up(operation);
        } else {
            self.record_applier_operation(operation);
        }
    }

    pub(crate) fn skip_to_end(&self) {}

    pub(crate) fn start_root(&mut self) {
        {
            let writer = &mut self.read_writer;
            writer.slot_index_stack.clear();
            writer.slot_visit_index = 0;
        }
        self.start_group(ROOT_KEY);
    }

    pub(crate) fn end_root(&mut self) {
        self.end_group(ROOT_KEY);
        {
            let writer = &mut self.read_writer;
            writer.slot_index_stack.clear();
            writer.slot_visit_index = 0;
        }
    }

    pub(crate) fn start_node(&mut self) {
        self.start(
            NODE_KEY,
            None,
            GroupKind::Node(),
            None);
        self.node_expected = true
    }

    pub(crate) fn end_group(&mut self, hash: u64) {
        self.end(false);
    }

    fn exit_group(&mut self, is_node: bool) {
        if is_node {
            self.node_index = self.node_index_stack.pop().unwrap();
        }
        self.read_writer.exit_group();
    }

    pub(crate) fn start_group(&mut self, hash: u64) {
        self.start(
            hash,
            None,
            GroupKind::Group {
                key: hash,
                depth: self.depth,
                skipping: false,
                slot_data: vec![].wrap_with_rc_refcell(),
            },
            None,
        );
    }

    pub(crate) fn create_node(&mut self) {
        self.validate_node_expected();

        {
            let insert_index_mut = self.node_index_stack.last_mut().unwrap();
            let insert_index = *insert_index_mut;
            *insert_index_mut += 1;

            drop(insert_index_mut);
            let slot = self.read_writer.parent();
            let slot_bottom_up = slot.clone();
            self.record_fix_up(move |applier, _| {
                let node = LayoutNode::new();
                slot.borrow_mut().update_node(node.clone());
                applier.insert_top_down(insert_index, node.clone());
                applier.down(node);
            });

            self.record_insert_up_fix_up(move |applier, _| {
                let node = slot_bottom_up.borrow().node();
                applier.up();
                applier.insert_bottom_up(insert_index, node.clone());
            });
        }
    }

    pub(crate) fn record_deferred_change(&mut self, deferred_change: impl FnOnce(&mut dyn Applier<ApplierInType>, &mut dyn RememberManager) + 'static) {
        self.composition.record_deferred_change(deferred_change);
    }

    pub(crate) fn apply_deferred_changes(&mut self) {
        self.composition.apply_deferred_changes();
    }

    pub(crate) fn use_node(&mut self) -> Rc<RefCell<LayoutNode>> {
        self.validate_node_expected();

        let node = self.read_writer.use_layout_node();
        self.read_writer.begin_use_layout_node(node.clone());

        let node_ref = node.clone();
        self.record_applier_operation(move |_, _| {
            node_ref.borrow_mut().on_reuse();
        });

        node
    }

    pub(crate) fn record_fix_up(&mut self, fix_up: impl FnOnce(&mut dyn Applier<Rc<RefCell<LayoutNode>>>, &mut dyn RememberManager) + 'static) {
        self.fix_up.push(Change {
            change: Box::new(fix_up),
            change_type: ChangeType::FixUp,
        });
        self.sequence += 1;
    }

    pub(crate) fn record_insert_up_fix_up(&mut self, insert_up_fix_up: impl FnOnce(&mut dyn Applier<Rc<RefCell<LayoutNode>>>, &mut dyn RememberManager) + 'static) {
        self.insert_up_fix_up.push(Change {
            change: Box::new(insert_up_fix_up),
            change_type: ChangeType::InsertUpFixUp,
        });
        self.sequence += 1;
    }

    pub(crate) fn register_insert_up_fix_up(&mut self) {
        if let Some(insert_up_fix_up) = self.insert_up_fix_up.pop() {
            self.fix_up.push(insert_up_fix_up)
        }
    }

    fn record_applier_operation(&mut self, action: impl FnOnce(&mut dyn Applier<ApplierInType>, &mut dyn RememberManager) + 'static) {
        self.composition.record(action)
    }

    fn record_slot_editing_operation(&mut self, action: impl FnOnce(&mut dyn Applier<ApplierInType>, &mut dyn RememberManager) + 'static) {
        self.composition.record(action)
    }

    fn record_insert(&mut self) {
        if self.fix_up.is_empty() {} else {
            let mut fix_ups: Vec<Change> = vec![];
            std::mem::swap(&mut fix_ups, &mut self.fix_up);

            self.record_slot_editing_operation(move |appiler, remember_manager| {
                fix_ups.into_iter().for_each(|change| {
                    (change.change)(appiler, remember_manager);
                });
            });
        }
    }

    pub(crate) fn end_node(&mut self) {
        self.end(true)
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
        println!("enter hash: {}", hash);
    }

    fn update_compound_hash_exit(&mut self, hash: u64) {
        self.depth -= 1;
        self.hash ^= hash;
        self.hash = self.hash.rotate_right(3);
        println!("exit hash: {}", hash);
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

        let is_node = group_kind.is_node();
        if self.inserting() {
            self.read_writer.begin_empty();

            if is_node {
                self.read_writer.start_node(key, None)
            } else if data.is_some() {
                todo!()
            } else {
                self.read_writer.start_group(key, self.depth, object_key)
            }

            self.enter_group(is_node);
            return;
        }
        
        if key == self.read_writer.group_key(self.read_writer.current_slot()) {
            todo!()
        } else {
            self.read_writer.begin_empty();
            self.inserting = true;
            if is_node {
                self.read_writer.start_node(key, None)
            } else if data.is_some() {
                todo!()
            } else {
                self.read_writer.start_group(key, self.depth, object_key)
            }
        }

        self.enter_group(group_kind.is_node());
    }

    fn enter_group(&mut self, is_node: bool) {
        if is_node {
            self.node_index_stack.push(self.node_index);
            self.node_index = 0;
        }

        self.read_writer.enter_group()
    }

    fn realize_movement(&mut self) {
        let count = self.previous_count;
        self.previous_count = 0;

        if count > 0 {
            if self.previous_remove >= 0 {
                let remove_index = self.previous_remove;
                self.previous_remove = -1;
                self.record_applier_operation(move |applier, _| {
                    applier.remove(remove_index as usize, count)
                });
            } else {
                todo!("move")
            }
        }
    }

    pub(crate) fn record_remove_node(&mut self, node_index: i32, count: usize) {
        if count > 0 {
            if self.previous_remove == node_index {
                self.previous_count += count
            } else {
                self.realize_movement();
                self.previous_remove = node_index;
                self.previous_count = count;
            }
        }
    }

    pub(crate) fn end(&mut self, is_node: bool) {
        let parent = self.read_writer.parent();
        self.update_compound_hash_exit(self.read_writer.group_key(Some(parent)));

        self.read_writer.end_empty();
        if is_node {
            if self.inserting() {
                self.read_writer.end_insert_layout_node();
                self.register_insert_up_fix_up();
                self.record_insert();
            } else {
                self.read_writer.end_insert_layout_node();
            }
        } else {
            if self.inserting() {
                self.read_writer.end_insert_group(GroupKindIndex::Group);

                if !self.read_writer.in_empty() {
                    self.inserting = false;
                }
            }

            let remove_index = self.read_writer.slot_visit_index as i32;
            while !self.read_writer.is_group_end() {
                let node_count = self.read_writer.skip_slot();
                self.record_remove_node(remove_index as i32, node_count);
            }
        }

        self.exit_group(is_node);
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

impl ComposerImpl {
    pub(crate) fn new(root: Rc<RefCell<LayoutNode>>) -> Self {
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
            changes: vec![],
            pending_stack: vec![],
            read_writer,
            invalidate_stack: vec![],
            composition: Composition::new(UiApplier::new(root)),

            previous_remove: -1,
            previous_count: 0,

            node_index: 0,
            node_index_stack: vec![],

            pending: None,
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