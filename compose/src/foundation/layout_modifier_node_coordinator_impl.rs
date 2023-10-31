use std::any::Any;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::{Rc, Weak};
use crate::foundation::constraint::Constraint;
use crate::foundation::geometry::{IntOffset, IntSize};
use crate::foundation::layout_modifier_node::LayoutModifierNode;
use crate::foundation::layout_modifier_node_coordinator::LayoutModifierNodeCoordinator;
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::layout_result::Placeable;
use crate::foundation::look_ahead_capable_placeable::{NodeCoordinator, NodeCoordinatorImpl};
use crate::foundation::measurable::Measurable;
use crate::foundation::measured::Measured;
use crate::foundation::modifier::Node;

impl LayoutModifierNodeCoordinator {
    pub(crate) fn new(layout_node: Weak<RefCell<LayoutNode>>, measure_node: Rc<RefCell<dyn Node>>) -> Self {
        Self {
            layout_node,
            node_coordinator_impl: NodeCoordinatorImpl::new(),
            layout_modifier_node:measure_node,
        }
    }

    pub(crate) fn set_layout_modifier_node(&mut self, mut layout_mod: Rc<RefCell<dyn Node>>) -> Rc<RefCell<dyn Node>>{
        std::mem::swap(&mut self.layout_modifier_node, &mut layout_mod);
        layout_mod
    }

    pub(crate) fn on_layout_modifier_node_changed(&self) {
        self.node_coordinator_impl.on_layout_modifier_node_changed()
    }
}

impl Measurable for LayoutModifierNodeCoordinator {
    fn measure(&mut self, constraint: &Constraint) -> &mut dyn Placeable {
        todo!()
    }
}

impl NodeCoordinator for LayoutModifierNodeCoordinator {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}