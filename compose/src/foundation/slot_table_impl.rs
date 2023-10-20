use crate::foundation::{SlotTable, SlotTableType};

impl SlotTable {
    pub(crate) fn push(&mut self, data: SlotTableType) -> &SlotTableType{
        self.data.insert(self.index, data);
            let result = &self.data[self.index ];
        self.index += 1;
        return   result;
    }
}

impl Default for SlotTable {
    fn default() -> Self {
        SlotTable {
            index: 0,
            data: Default::default()
        }
    }
}