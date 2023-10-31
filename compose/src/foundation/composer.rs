use std::{rc::Rc, cell::RefCell};

use super::{layout_node::LayoutNode, slot_table::SlotTable};

#[derive( Default)]
pub(crate) struct ComposerInner {
    pub hash: i64,
    pub depth: usize,

    pub(crate) inserting: bool,
    pub(crate) layout_node_stack: Vec<Rc<RefCell<LayoutNode>>>,
    pub(crate) slot_table: SlotTable,
    pub(crate) root: Option<Rc<RefCell<LayoutNode>>>,

    pub(crate) fix_up: Vec<Box<dyn FnOnce()>>,
    pub(crate) insert_up_fix_up: Vec<Box<dyn FnOnce()>>,
    pub(crate) deferred_changes: Vec<Box<dyn FnOnce()>>
}

pub struct Composer {
    pub(crate) inner: RefCell<ComposerInner>
}

#[macro_export]
macro_rules! run_compose {
    ($function:ident, $composer:expr) => {
        concat_idents!(__, $function, __compose_synthesis__)($composer);
    };

    ($function:ident) => {
        concat_idents!(__, $function, __compose_synthesis__)(composer);
    }
}