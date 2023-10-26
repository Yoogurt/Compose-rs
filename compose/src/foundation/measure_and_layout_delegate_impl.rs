use std::{cell::RefCell, rc::Rc};

use crate::foundation::measurable::MultiChildrenMeasurePolicy;

use super::{
    constraint::Constraint, layout_node::LayoutNode,
    measure_and_layout_delegate::MeasureAndLayoutDelegate,
};

impl MeasureAndLayoutDelegate {
    pub(crate) fn new() -> Self {
        MeasureAndLayoutDelegate {
            root: LayoutNode::new(),
            root_constraint: Constraint::unbounded(),
            during_measure_layout: false,
        }
    }

    pub(crate) fn update_root_constraints(&mut self, constraint: Constraint) {
        if constraint == self.root_constraint {
            dbg!("constraint the same, skip measuring");
            return;
        }

        self.root_constraint = constraint;

        let root_mut = self.root.borrow_mut();
        root_mut.as_remeasurable().borrow_mut().mark_remeasure_pending();
    }

    pub(crate) fn update_root_measure_policy(&self, measure_policy: MultiChildrenMeasurePolicy) {
        self.root.borrow_mut().set_measure_policy(measure_policy);
    }

    fn perform_measure_and_layout(&mut self, block: impl FnOnce(&mut Self)) {
        self.during_measure_layout = true;
        block(self);
        self.during_measure_layout = false;
    }

    fn do_remeasure(layout_node: &LayoutNode, constraint: Option<Constraint>) -> bool {
        let mut size_changed = false;

        layout_node.as_remeasurable().borrow_mut().remeasure(constraint);

        size_changed
    }

    fn remeasure_only(&self, layout_node: Rc<RefCell<LayoutNode>>) {
        let constraint = if std::ptr::eq(&*layout_node, &*self.root) {
            Some(self.root_constraint)
        } else {
            None
        };
    }

    fn recurse_remeasure(&self) {
        self.remeasure_only(self.root.clone());
    }

    pub(crate) fn measure_only(&mut self) {
        self.perform_measure_and_layout(|self_| {
            self_.recurse_remeasure();
        });
    }
}
