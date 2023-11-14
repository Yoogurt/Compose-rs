use std::cell::{RefCell, RefMut};
use std::cell::Ref;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ops::Deref;
use std::ops::Index;
use std::rc::Rc;

use crate::foundation::layout_node::LayoutNode;
use crate::foundation::slot_table_type::SlotTableType;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;

use super::slot_table_type::{GroupKind, GroupKindIndex};

#[derive(Debug, Default)]
pub(crate) struct SlotTable {
    pub(crate) slots: Rc<RefCell<Vec<SlotTableType>>>,
    // pub(crate) slots_stack: Vec<>

    pub(crate) groups: Vec<usize>,
    pub(crate) groups_size: usize,

    // at most 1, we do not support multiple composer reader/writer
    pub(crate) readers: usize,
    pub(crate) writer: usize,
}

pub(crate) struct SlotReader {
    slot: Rc<RefCell<Vec<SlotTableType>>>,
    empty_count: usize,
    current_slot: usize,
}

impl SlotReader {
    pub(crate) fn new(slot: Rc<RefCell<Vec<SlotTableType>>>) -> Self {
        Self {
            slot,
            empty_count: 0,
            current_slot: 0,
        }
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
}

pub(crate) struct SlotWriter {
    pub(crate) slot: Rc<RefCell<Vec<SlotTableType>>>,
    pub(crate) slot_stack: Vec<Rc<RefCell<Vec<SlotTableType>>>>,
}

impl SlotWriter {
    pub(crate) fn new(slot: Rc<RefCell<Vec<SlotTableType>>>) -> Self {
        Self {
            slot,
            slot_stack: vec![],
        }
    }

    pub(crate) fn begin_insert_group(&mut self, hash: i64, depth: usize) {
        let group_kind = GroupKind::Group {
            hash,
            depth,
            slot_data: vec![].wrap_with_rc_refcell(),
        };

        self.slot.borrow_mut().push(SlotTableType {
            data: group_kind,
        });
    }

    pub(crate) fn enter_group(&mut self) {
        self.slot_stack.push(self.slot.clone());
        let slot = if let Some(slot_table_type) = self.slot.borrow().last() {
            match slot_table_type.data {
                GroupKind::Group { ref slot_data, .. } => slot_data.clone(),
                _ => panic!("not a group"),
            }
        } else {
            panic!("no group found")
        };

        self.slot = slot;
    }

    pub(crate) fn end_insert_group(&mut self, group_kind_index: GroupKindIndex) {}

    pub(crate) fn exit_group(&mut self) {
        self.slot = self.slot_stack.pop().unwrap();
    }

    pub(crate) fn begin_insert_layout_node(&mut self, layout_node: Rc<RefCell<LayoutNode>>) {
        let group_kind = GroupKind::LayoutNodeType(layout_node);
        let child_index = self.slot.borrow().len();

        self.slot.borrow_mut().push(SlotTableType {
            data: group_kind,
        });
    }

    pub(crate) fn get_group_kind<'a, 'b>(
        &mut self,
        group_kind_index: GroupKindIndex,
        parent_stack: &'a mut RefMut<'b, Vec<SlotTableType>>,
    ) -> Option<&'a mut GroupKind> {
        parent_stack.iter_mut().rev().find_map(|slot_table_type| {
            let group_kind = &mut slot_table_type.data;
            if group_kind.index() == group_kind_index {
                Some(group_kind)
            } else {
                None
            }
        })
    }

    pub(crate) fn end_insert_layout_node(&mut self) {}

    pub(crate) fn validate(&self) {
        if !self.slot_stack.is_empty() {
            panic!("unbalanced slot stack")
        }
    }

    pub(crate) fn slot_stack(&self) -> Rc<RefCell<Vec<SlotTableType>>> {
        self.slot_stack.last().unwrap().clone()
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
    pub(crate) fn open_reader(&mut self) -> SlotReader {
        self.readers += 1;
        SlotReader::new(self.slots.clone())
    }

    pub(crate) fn open_writer(&mut self) -> SlotWriter {
        self.writer += 1;
        SlotWriter::new(self.slots.clone())
    }
}
