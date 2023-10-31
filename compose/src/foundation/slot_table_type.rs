use std::{cell::RefCell, rc::Rc, any::Any};

use super::{layout_node::LayoutNode, slot_table::SlotTable};

#[derive(Debug)]
pub(crate) enum GroupKind {
    SlotTableType(SlotTable),
    Group {
        hash: i64,
        depth: usize
    },
    LayoutNodeType(Rc<RefCell<LayoutNode>>),
    CustomType(Box<dyn Any>)
}