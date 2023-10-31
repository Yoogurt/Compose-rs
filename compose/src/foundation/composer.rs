use std::{rc::Rc, cell::RefCell};

use super::{layout_node::LayoutNode, slot_table::SlotTable};

#[derive(Default)]
pub(crate) struct ComposerInner {
    pub(crate) hash: i64,
    pub(crate) depth: usize,
    pub(crate) node_expected: bool,

    pub(crate) inserting: bool,
    pub(crate) layout_node_stack: Vec<Rc<RefCell<LayoutNode>>>,
    pub(crate) slot_table: SlotTable,
    pub(crate) root: Option<Rc<RefCell<LayoutNode>>>,

    pub(crate) fix_up: Vec<Box<dyn FnOnce()>>,
    pub(crate) insert_up_fix_up: Vec<Box<dyn FnOnce()>>,
    pub(crate) deferred_changes: Vec<Box<dyn FnOnce()>>,
}

pub struct Composer {
    pub(crate) inner: RefCell<ComposerInner>,
}