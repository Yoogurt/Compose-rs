use std::any::Any;
use std::cell::RefCell;
use std::cell::Ref;
use std::ops::Deref;
use std::rc::Rc;
use crate::foundation::composer_impl::NODE_KEY;

use crate::foundation::layout_node::LayoutNode;
use crate::foundation::slot_table_type;
use crate::foundation::slot_table_type::Slot;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;

use super::slot_table_type::{GroupKind, GroupKindIndex};

#[derive(Debug, Default)]
pub(crate) struct SlotTable {
    pub(crate) slots: Rc<RefCell<Vec<Slot>>>,
    // pub(crate) slots_stack: Vec<>

    pub(crate) groups: Vec<usize>,
    pub(crate) groups_size: usize,

    // at most 1, we do not support multiple composer reader/writer
    pub(crate) readers: usize,
    pub(crate) writer: usize,
}

pub(crate) struct SlotReadWriter {
    empty_count: usize,
    slot: Rc<RefCell<Vec<Slot>>>,
    pub(crate) slot_visit_index: usize,
    pub(crate) slot_index_stack: Vec<usize>,
    pub(crate) slot_stack: Vec<Rc<RefCell<Vec<Slot>>>>,

    pub(crate) current_layout_node: Option<Rc<RefCell<LayoutNode>>>,
    pub(crate) layout_node_stack: Vec<Rc<RefCell<LayoutNode>>>,
}

impl SlotReadWriter {
    pub(crate) fn new(slot: Rc<RefCell<Vec<Slot>>>) -> Self {
        Self {
            empty_count: 0,
            slot,
            slot_visit_index: 0,
            slot_index_stack: vec![],
            slot_stack: vec![],
            current_layout_node: None,
            layout_node_stack: vec![],
        }
    }

    pub(crate) fn in_empty(&self) -> bool {
        self.empty_count > 0
    }

    pub(crate) fn begin_empty(&mut self) {
        self.empty_count += 1;
    }

    pub(crate) fn end_empty(&mut self) {
        if self.empty_count == 0 {
            panic!("unbalanced begin/end empty")
        }

        self.empty_count -= 1;
    }

    pub(crate) fn parent(&self) -> Slot {
        let last_index = *self.slot_index_stack.last().unwrap();
        self.slot_stack.last().unwrap().borrow().get(last_index - 1).cloned().unwrap()
    }

    pub(crate) fn skipping(&self) -> bool {
        self.slot.borrow().first().map(|group| match group.borrow().deref() {
            GroupKind::Group { skipping, .. } => {
                *skipping
            }
            _ => {
                panic!()
            }
        }).unwrap_or(false)
    }

    fn insert_slot(&mut self, slot_table_data: impl Into<Slot>) {
        self.slot.borrow_mut().insert(self.slot_visit_index, slot_table_data.into());
        self.skip_slot();
    }

    fn replace_slot(&mut self, slot_table_type: Slot) -> Slot {
        let slot_table_type = std::mem::replace(&mut self.slot.borrow_mut()[self.slot_visit_index], slot_table_type);
        self.skip_slot();
        slot_table_type
    }

    pub(crate) fn start_node(&mut self, key: u64, object_key: Option<Box<dyn Any>>) {
        self.insert_slot(GroupKind::Node())
    }

    pub(crate) fn group_key(&self, slot: Option<Slot>) -> u64 {
        if slot.is_none() {
            return 0
        }

        match slot.unwrap().borrow().deref() {
            GroupKind::Group {
                key, ..
            } => {
                *key
            }
            GroupKind::Node { .. } => {
                NODE_KEY
            }
            _ => {
                panic!("not key group found")
            }
        }
    }

    pub(crate) fn start_group(&mut self, key: u64, depth: usize, object_key: Option<Box<dyn Any>>) {
        self.insert_slot(GroupKind::Group {
            key,
            depth,
            skipping: false,
            slot_data: vec![].wrap_with_rc_refcell(),
        })
    }

    pub(crate) fn current_slot(&self) -> Option<Slot> {
        self.slot.borrow().get(self.slot_visit_index).cloned()
    }

    pub(crate) fn begin_insert_group(&mut self, hash: u64, depth: usize) {
        let group_kind = GroupKind::Group {
            key: hash,
            depth,
            skipping: false,
            slot_data: vec![].wrap_with_rc_refcell(),
        };

        self.insert_slot(group_kind.wrap_with_rc_refcell());
    }

    pub(crate) fn replace_group(&mut self, hash: u64, depth: usize) -> Slot {
        let group_kind = GroupKind::Group {
            key: hash,
            depth,
            skipping: false,
            slot_data: vec![].wrap_with_rc_refcell(),
        };

        self.replace_slot(group_kind.wrap_with_rc_refcell())
    }

    pub(crate) fn enter_group(&mut self) {
        let slot = self.slot.borrow().get(self.slot_visit_index - 1).and_then(|slot| {
            match slot.borrow().deref() {
                GroupKind::Group {
                    slot_data, ..
                } => {
                    Some(slot_data.clone())
                }
                GroupKind::Node {
                    slot_data, ..
                } => {
                    Some(slot_data.clone())
                }
                _ => {
                    None
                }
            }
        });

        self.slot_stack.push(self.slot.clone());
        self.slot = slot.unwrap();
        self.slot_index_stack.push(self.slot_visit_index);

        self.slot_visit_index = 0;
    }

    pub(crate) fn end_insert_group(&mut self, group_kind_index: GroupKindIndex) {}

    pub(crate) fn exit_group(&mut self) {
        self.slot = self.slot_stack.pop().unwrap();
        self.slot_visit_index = self.slot_index_stack.pop().unwrap();
    }

    pub(crate) fn begin_use_layout_node(&mut self, layout_node: Rc<RefCell<LayoutNode>>) {
        if let Some(pre_layout_node) = self.current_layout_node.as_ref() {
            self.layout_node_stack.push(pre_layout_node.clone());
        }
        self.current_layout_node = Some(layout_node.clone());
    }

    pub fn use_layout_node(&mut self) -> Rc<RefCell<LayoutNode>> {
        let node = match self.current_slot().unwrap().borrow().deref() {
            GroupKind::Node { node, .. } => node.as_ref().unwrap().clone(),
            _ => panic!("not a layout node"),
        };
        self.skip_slot();

        node
    }

    pub(crate) fn parent_layout_node(&self) -> Option<Rc<RefCell<LayoutNode>>> {
        self.layout_node_stack.last().cloned()
    }

    pub(crate) fn end_insert_layout_node(&mut self) {
        self.current_layout_node = self.layout_node_stack.pop();
    }

    pub(crate) fn validate(&self) {
        if !self.slot_stack.is_empty() {
            panic!("unbalanced slot stack")
        }
    }

    pub fn skip_slot(&mut self) -> usize {
        self.slot_visit_index += 1;
        1
    }

    pub fn pop_current_slot(&mut self) -> Slot {
        self.slot.borrow_mut().remove(self.slot_visit_index)
    }

    pub(crate) fn update(&mut self, value: Rc<RefCell<dyn Any>>) {
        self.insert_slot(GroupKind::CustomType(value));
    }

    pub(crate) fn is_group_end(&self) -> bool {
        self.slot.borrow().len() - self.slot_visit_index == 0
    }
}

pub(crate) struct GroupKindBorrowGuard<'a> {
    slot: Ref<'a, Vec<GroupKind>>,
    index: usize,
}

impl<'a> GroupKindBorrowGuard<'a> {
    fn new(slot: Ref<'a, Vec<GroupKind>>, index: usize) -> Self {
        Self { slot, index }
    }
}

impl Deref for GroupKindBorrowGuard<'_> {
    type Target = GroupKind;

    fn deref(&self) -> &Self::Target {
        &self.slot[self.index]
    }
}

impl SlotTable {
    pub(crate) fn open_read_writer(&mut self) -> SlotReadWriter {
        self.writer += 1;
        SlotReadWriter::new(self.slots.clone())
    }
}
