use super::slot_table_type::GroupKind;

#[derive(Debug)]
pub(crate) struct SlotTable {
    pub(crate) data: Vec<GroupKind>,
    pub(crate) index: usize,

    // at most 1, we do not support multiple composer reader/writer
    pub(crate) readers: usize,
    pub(crate) writer: usize,
}