use std::cell::{RefCell, RefMut};
use std::cell::Ref;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ops::Deref;
use std::ops::Index;
use std::rc::Rc;

use crate::foundation::layout_node::LayoutNode;
use crate::foundation::slot_table_type::SlotTableType;

use super::slot_table_type::{GroupKind, GroupKindIndex};

#[derive(Debug, Default)]
pub(crate) struct SlotTable {
    pub(crate) slots: Rc<RefCell<Vec<SlotTableType>>>,
    pub(crate) groups: Vec<usize>,
    pub(crate) groups_size: usize,

    // at most 1, we do not support multiple composer reader/writer
    pub(crate) readers: usize,
    pub(crate) writer: usize,
}

pub(crate) struct SlotReader {
    slot: Rc<RefCell<Vec<SlotTableType>>>,
    empty_count: usize,
}

impl SlotReader {
    pub(crate) fn new(slot: Rc<RefCell<Vec<SlotTableType>>>) -> Self {
        Self {
            slot,
            empty_count: 0,
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
    pub(crate) group_parent: HashMap<GroupKindIndex, Vec<usize>>,
}

impl SlotWriter {
    pub(crate) fn new(slot: Rc<RefCell<Vec<SlotTableType>>>) -> Self {
        Self {
            slot,
            group_parent: Default::default(),
        }
    }

    pub(crate) fn begin_insert_layout_node(&mut self, layout_node: Rc<RefCell<LayoutNode>>) {
        let group_kind = GroupKind::LayoutNodeType(layout_node);
        let child_index = self.slot.borrow().len();

        let parent = match self.group_parent.entry(group_kind.index()) {
            Entry::Occupied(mut entry) => {
                let parent = *entry.get().last().unwrap_or(&0);
                entry.get_mut().push(child_index);
                parent
            }
            Entry::Vacant(mut entry) => {
                entry.insert(vec![child_index]);
                0
            }
        };

        self.slot.borrow_mut().push(SlotTableType {
            data: group_kind,
            parent,
        });
    }

    pub(crate) fn get_group_kind<'a, 'b>(
        &mut self,
        group_kind: GroupKindIndex,
        data: &'a mut RefMut<'b, Vec<SlotTableType>>,
    ) -> Option<&'a mut GroupKind> {
        let parent = self.group_parent.get_mut(&group_kind);
        match parent {
            None => None,
            Some(stack) => match stack.last() {
                None => None,
                Some(current) => match data.get_mut(*current) {
                    None => None,
                    Some(node) => Some(&mut node.data),
                },
            },
        }
    }

    pub(crate) fn end_insert_layout_node(&mut self) {
        self.group_parent
            .get_mut(&GroupKindIndex::LayoutNode)
            .unwrap()
            .pop();
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
