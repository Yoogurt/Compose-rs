#[derive(Debug)]
pub(crate) struct SlotTable {
    data: Vec<SlotTableType>,
        index: usize,
}