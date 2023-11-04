use super::{constraint::Constraints, layout_node::LayoutNode};
use crate::foundation::measurable::MultiChildrenMeasurePolicy;
use std::{cell::RefCell, rc::Rc};

pub struct MeasureAndLayoutDelegate {
    pub(crate) root: Rc<RefCell<LayoutNode>>,
    pub(crate) root_constraint: Constraints,
    pub(crate) during_measure_layout: bool,
}

impl MeasureAndLayoutDelegate {
    pub(crate) fn new() -> Self {
        MeasureAndLayoutDelegate {
            root: LayoutNode::new(),
            root_constraint: Constraints::unbounded(),
            during_measure_layout: false,
        }
    }

    pub(crate) fn update_root_constraints(&mut self, constraint: Constraints) {
        if constraint == self.root_constraint {
            dbg!("constraint the same, skip measuring");
            return;
        }

        self.root_constraint = constraint;

        let root_mut = self.root.borrow_mut();
        root_mut
            .layout_node_layout_delegate
            .borrow_mut()
            .measure_pass_delegate
            .borrow_mut()
            .mark_measure_pending();
    }

    pub(crate) fn update_root_measure_policy(&self, measure_policy: MultiChildrenMeasurePolicy) {
        self.root.borrow_mut().set_measure_policy(measure_policy);
    }

    fn perform_measure_and_layout(&mut self, block: impl FnOnce(&mut Self)) {
        self.during_measure_layout = true;
        block(self);
        self.during_measure_layout = false;
    }

    fn remeasure_only(&self, layout_node: Rc<RefCell<LayoutNode>>) {
        let constraint = if std::ptr::eq(&*layout_node, &*self.root) {
            Some(self.root_constraint)
        } else {
            None
        };

        let layout_node_layout_delegate = layout_node.borrow().layout_node_layout_delegate.clone();
        layout_node_layout_delegate
            .borrow_mut()
            .remeasure(constraint);
    }

    fn recurse_remeasure(&self, layout_node: Rc<RefCell<LayoutNode>>) {
        self.remeasure_only(layout_node.clone());

        layout_node.borrow().for_each_child(|child| {
            self.recurse_remeasure(child.clone());
        });

        self.remeasure_only(layout_node);
    }

    pub(crate) fn measure_only(&mut self) {
        self.perform_measure_and_layout(|self_| {
            self_.recurse_remeasure(self_.root.clone());
        });
    }
}
