use crate::foundation::slot_table::{SlotReader, SlotWriter};
use super::{slot_table_type::GroupKind, slot_table::SlotTable};
use std::cell::Ref;
use std::ops::Deref;

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