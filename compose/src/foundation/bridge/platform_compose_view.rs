use std::cell::RefCell;
use std::rc::Rc;
use crate::foundation::bridge::root_measure_policy::root_measure_policy;
use crate::foundation::canvas::Canvas;
use crate::foundation::constraint::Constraint;
use crate::foundation::layout_node::LayoutNode;

pub struct MacOSComposeView {
    root: Rc<RefCell<LayoutNode>>,
    root_constraint: Constraint,
}

impl MacOSComposeView {
    pub fn new() -> MacOSComposeView {
        let root_layout_node = LayoutNode::new();

        let result = MacOSComposeView {
            root: root_layout_node,
            root_constraint: Constraint::fixed(0, 0),
        };

        result.root.borrow().set_measure_policy(root_measure_policy());
        result
    }

    pub fn dispatch_measure(&mut self, width: usize, height: usize) {
        let constraint = Constraint::new(0..=width, 0..=height);
        if constraint == self.root_constraint {
            dbg!("constraint the same, skip measuring");
            return;
        }

        self.root_constraint = constraint;
        self.root.borrow().remeasure().borrow_mut().remeasure(&self.root_constraint);
    }

    pub fn dispatch_draw(&mut self, _canvas: &dyn Canvas) {}
}