use std::any::Any;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::rc::{Rc, Weak};
use crate::foundation::constraint::Constraint;
use crate::foundation::layout_modifier_node_coordinator::LayoutModifierNodeCoordinator;
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::placeable::Placeable;
use crate::foundation::node_coordinator::{NodeCoordinator, NodeCoordinatorImpl};
use crate::foundation::measurable::Measurable;
use crate::foundation::modifier::Node;

impl DerefMut for LayoutModifierNodeCoordinator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node_coordinator_impl
    }
}

impl Deref for LayoutModifierNodeCoordinator {
    type Target = dyn NodeCoordinator;

    fn deref(&self) -> &Self::Target {
        &self.node_coordinator_impl
    }
}

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