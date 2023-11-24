use std::any::Any;
use std::cell::RefCell;
use std::cell::Ref;
use std::ops::Deref;
use std::rc::Rc;

use crate::foundation::layout_node::LayoutNode;
use crate::foundation::slot_table_type;
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

pub(crate) struct SlotReadWriter {
    empty_count: usize,
    pub(crate) slot: Rc<RefCell<Vec<SlotTableType>>>,
    pub(crate) current_slot_index: usize,
    pub(crate) slot_index_stack: Vec<usize>,
    pub(crate) slot_stack: Vec<Rc<RefCell<Vec<SlotTableType>>>>,

    pub(crate) current_layout_node: Option<Rc<RefCell<LayoutNode>>>,
    pub(crate) layout_node_stack: Vec<Rc<RefCell<LayoutNode>>>,
}

impl SlotReadWriter {
    pub(crate) fn new(slot: Rc<RefCell<Vec<SlotTableType>>>) -> Self {
        Self {
            empty_count: 0,
            slot,
            current_slot_index: 0,
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

    fn insert_slot_table_type(&mut self, slot_table_type: SlotTableType) {
        self.slot.borrow_mut().insert(self.current_slot_index, slot_table_type);
        self.skip_slot();
    }

    fn replace_slot_table_type(&mut self, slot_table_type: SlotTableType) -> SlotTableType {
        let slot_table_type = std::mem::replace(&mut self.slot.borrow_mut()[self.current_slot_index], slot_table_type);
        self.skip_slot();
        slot_table_type
    }

    pub(crate) fn begin_insert_group(&mut self, hash: u64, depth: usize) {
        let group_kind = GroupKind::Group {
            hash,
            depth,
            skipping: false,
            slot_data: vec![].wrap_with_rc_refcell(),
        };

        self.insert_slot_table_type(SlotTableType {
            data: group_kind,
        });
    }

    pub(crate) fn replace_group(&mut self, hash: u64, depth: usize) -> SlotTableType {
        let group_kind = GroupKind::Group {
            hash,
            depth,
            skipping: false,
            slot_data: vec![].wrap_with_rc_refcell(),
        };

        self.replace_slot_table_type(SlotTableType {
            data: group_kind,
        })
    }

    pub(crate) fn enter_group(&mut self) {
        self.slot_stack.push(self.slot.clone());
        let slot = if let Some(slot_table_type) = self.slot.borrow().get(self.current_slot_index - 1) {
            match slot_table_type.data {
                GroupKind::Group { ref slot_data, .. } => slot_data.clone(),
                _ => panic!("not a group"),
            }
        } else {
            panic!("no group found")
        };

        self.slot = slot;
        self.slot_index_stack.push(self.current_slot_index);
        self.current_slot_index = 0;
    }

    pub(crate) fn end_insert_group(&mut self, group_kind_index: GroupKindIndex) {}

    pub(crate) fn exit_group(&mut self) {
        self.slot = self.slot_stack.pop().unwrap();
        self.current_slot_index = self.slot_index_stack.pop().unwrap();
    }

    pub(crate) fn begin_insert_layout_node(&mut self, layout_node: Rc<RefCell<LayoutNode>>) {
        let group_kind = GroupKind::LayoutNodeType(layout_node.clone());

        self.insert_slot_table_type(SlotTableType {
            data: group_kind,
        });

        self.begin_use_layout_node(layout_node);
    }

    pub(crate) fn begin_use_layout_node(&mut self, layout_node: Rc<RefCell<LayoutNode>>) {
        if let Some(pre_layout_node) = self.current_layout_node.as_ref() {
            self.layout_node_stack.push(pre_layout_node.clone());
        }
        self.current_layout_node = Some(layout_node.clone());
    }

    pub fn use_layout_node(&mut self) -> Rc<RefCell<LayoutNode>> {
        let node = match self.slot.borrow()[self.current_slot_index].data {
            GroupKind::LayoutNodeType(ref layout_node) => layout_node.clone(),
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

    pub fn skip_slot(&mut self) {
        self.current_slot_index += 1;
    }

    pub fn pop_current_slot(&mut self) -> SlotTableType {
        self.slot.borrow_mut().remove(self.current_slot_index)
    }

    pub(crate) fn update(&mut self, value: Rc<RefCell<dyn Any>>) {
        self.insert_slot_table_type(SlotTableType {
            data: GroupKind::CustomType(value),
        });
    }

    pub(crate) fn is_group_end(&self) -> bool {
        self.slot.borrow().len() - self.current_slot_index == 0
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
