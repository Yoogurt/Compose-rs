use std::any::Any;

pub struct Composer {
    pub hash: RefCell<i64>,
    pub depth: RefCell<usize>,

    pub(crate) slot_table: Vec<Box<dyn Any>>,
}