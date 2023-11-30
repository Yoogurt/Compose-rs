use std::{cell::RefCell, rc::Rc};

use crate::foundation::measurable::MultiChildrenMeasurePolicy;
use crate::foundation::placeable_place_at::PlaceablePlaceAt;

use super::{constraint::Constraints, layout_node::LayoutNode};

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

    fn remeasure_and_relayout_if_need(&self, layout_node: Rc<RefCell<LayoutNode>>) {
        let layout_node_mut = layout_node.borrow_mut();

        let layout_node_layout_delegate = layout_node_mut.layout_node_layout_delegate.clone();
        let measure_pass_delegate = layout_node_layout_delegate.borrow().measure_pass_delegate.clone();
        let layout_pending = measure_pass_delegate.borrow().layout_pending;
        let is_placed = measure_pass_delegate.borrow().is_placed;

        drop(layout_node_mut);
        if layout_pending && is_placed {
            if std::ptr::eq(layout_node.as_ptr(), self.root.as_ptr()) {
                measure_pass_delegate.borrow_mut().place_at((0, 0).into(), 0.0)
            } else {}
        }
    }

    pub(crate) fn measure_only(&mut self) {
        self.perform_measure_and_layout(|this| {
            this.recurse_remeasure(this.root.clone());
        });
    }

    pub(crate) fn measure_and_layout(&mut self) {
        self.perform_measure_and_layout(|this| {
            this.remeasure_and_relayout_if_need(this.root.clone());
        });
    }
}
