use super::slot_table_type::SlotTableType;

#[derive(Debug)]
pub(crate) struct SlotTable {
    pub(crate) data: Vec<SlotTableType>,
    pub(crate) index: usize,
}
