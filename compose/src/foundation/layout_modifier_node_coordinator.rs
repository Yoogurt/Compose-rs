use std::cell::RefCell;
use std::rc::{Rc, Weak};
use auto_delegate::Delegate;
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::node_coordinator::{NodeCoordinator};
use crate::foundation::modifier::Node;
use std::any::Any;
use std::ops::{DerefMut, Deref};
use crate::foundation::constraint::Constraint;
use crate::foundation::layout_modifier_node::LayoutModifierNode;
use crate::foundation::layout_modifier_node_impl::LayoutModifierNodeImpl;
use crate::foundation::placeable::Placeable;
use crate::foundation::measurable::Measurable;
use crate::foundation::node_coordinator_impl::NodeCoordinatorImpl;

#[derive(Debug, Delegate)]
pub(crate) struct LayoutModifierNodeCoordinator {
    pub(crate) layout_node: Weak<RefCell<LayoutNode>>,
    pub(crate) layout_modifier_node: Rc<RefCell<LayoutModifierNodeImpl>>,
    #[to(Placeable, Measured, NodeCoordinatorTrait, MeasureScope, PlaceablePlaceAt)]
    pub(crate) node_coordinator_impl: NodeCoordinatorImpl,
}

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
    pub(crate) fn new(layout_node: Weak<RefCell<LayoutNode>>, measure_node: Rc<RefCell<LayoutModifierNodeImpl>>) -> Self {
        Self {
            layout_node,
            node_coordinator_impl: NodeCoordinatorImpl::new(),
            layout_modifier_node: measure_node,
        }
    }

    pub(crate) fn set_layout_modifier_node(&mut self, mut layout_mod: Rc<RefCell<dyn Node>>) -> Rc<RefCell<dyn Node>> {
        std::mem::swap(&mut self.layout_modifier_node, &mut layout_mod);
        layout_mod
    }

    pub(crate) fn on_layout_modifier_node_changed(&self) {
        self.node_coordinator_impl.on_layout_modifier_node_changed()
    }
}

impl Measurable for LayoutModifierNodeCoordinator {
    fn measure(&mut self, constraint: &Constraint) -> &mut dyn Placeable {
        self.perform_measure(constraint, move |self_| {
            let layout_modifier_node = self.layout_modifier_node.clone();

self_
        });

        self.on_measured();
        self
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