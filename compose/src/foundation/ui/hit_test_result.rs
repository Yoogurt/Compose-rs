use std::cell::RefCell;
use std::rc::Rc;
use std::ops::{Deref, DerefMut};
use crate::foundation::modifier::ModifierNode;

pub(crate) struct HitTestResult {
    values: Vec<Rc<RefCell<dyn ModifierNode>>>,
    hit_depth: i32,
}

impl HitTestResult {
    pub(crate) fn new() -> Self {
        Self {
            values: vec![],
            hit_depth: -1,
        }
    }

    pub(crate) fn accept_hits(&mut self) {
        self.hit_depth = self.len() as i32 - 1;
    }
}

impl Deref for HitTestResult {
    type Target = Vec<Rc<RefCell<dyn ModifierNode>>>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl DerefMut for HitTestResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

