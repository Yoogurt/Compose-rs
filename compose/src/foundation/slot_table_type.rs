#[derive(Debug)]
pub(crate) enum SlotTableType {
    SlotTableType(SlotTable),
    Group {
        hash: i64,
         depth: usize
    },
    LayoutNodeType(Rc<RefCell<LayoutNode>>),
    CustomType(Box<dyn Any>)
}