use crate::foundation::bridge::root_measure_policy::root_measure_policy;
use crate::foundation::canvas::Canvas;
use crate::foundation::constraint::Constraint;
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::measure_and_layout_delegate::MeasureAndLayoutDelegate;
use std::cell::RefCell;
use std::rc::Rc;

pub struct MacOSComposeView {
    measure_and_layout_delegate: MeasureAndLayoutDelegate,
}

impl MacOSComposeView {
    pub fn new() -> MacOSComposeView {
        let root_layout_node = LayoutNode::new();

        let result = MacOSComposeView {
            measure_and_layout_delegate: MeasureAndLayoutDelegate::new(),
        };

        result
            .measure_and_layout_delegate
            .update_root_measure_policy(root_measure_policy());
        result
    }

    pub fn dispatch_measure(&mut self, width: usize, height: usize) {
        let constraint = Constraint::new(0..=width, 0..=height);
        self.measure_and_layout_delegate
            .update_root_constraints(constraint);
        self.measure_and_layout_delegate.measure_only();
    }

    pub fn dispatch_draw(&mut self, _canvas: &dyn Canvas) {}
}
