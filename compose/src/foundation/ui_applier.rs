use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::foundation::applier::{AbstractApplier, Applier};
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::utils::option_extension::OptionThen;

pub(crate) struct DefaultUiApplier {
    applier_impl: AbstractApplier<Rc<RefCell<LayoutNode>>>,
}

impl Deref for DefaultUiApplier {
    type Target = AbstractApplier<Rc<RefCell<LayoutNode>>>;

    fn deref(&self) -> &Self::Target {
        &self.applier_impl
    }
}

impl Applier<Rc<RefCell<LayoutNode>>> for DefaultUiApplier {
    fn get_current(&self) -> &Rc<RefCell<LayoutNode>> {
        self.applier_impl.get_current()
    }

    fn on_end_changes(&self) {
        self.root.borrow_mut().owner.as_ref().and_then(|owner| {
            owner.upgrade()
        }).then(|owner| owner.borrow_mut().on_end_apply_changes());
    }

    fn down(&mut self, node: Rc<RefCell<LayoutNode>>) {
        self.applier_impl.down(node);
    }

    fn up(&mut self) {
        self.applier_impl.up()
    }

    fn clear(&mut self) {
        self.applier_impl.clear();
        self.applier_impl.root.borrow_mut().remove_all();
    }

    fn insert_top_down(&self) {
        todo!()
    }

    fn insert_bottom_up(&self) {
        todo!()
    }

    fn remove(&self, index: usize, count: usize) {
        todo!()
    }

    fn r#move(&self, from: usize, to: usize, count: usize) {
        todo!()
    }
}