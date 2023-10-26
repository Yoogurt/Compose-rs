use std::{cell::RefCell, rc::Rc};

use super::{constraint::Constraint, layout_node::LayoutNode};

pub struct MeasureAndLayoutDelegate {
    pub(crate) root: Rc<RefCell<LayoutNode>>,
    pub(crate) root_constraint:Constraint,
    pub(crate) during_measure_layout: bool,
}