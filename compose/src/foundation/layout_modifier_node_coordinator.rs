use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

use auto_delegate::Delegate;
use compose_foundation_macro::AnyConverter;

use crate::foundation::constraint::Constraints;
use crate::foundation::delegatable_node::ToDelegatedNode;
use crate::foundation::geometry::IntSize;
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::measurable::Measurable;
use crate::foundation::modifier::ModifierNode;
use crate::foundation::node_chain::NodeChain;
use crate::foundation::node_coordinator::{NodeCoordinator, NodeCoordinatorTrait, PerformDrawTrait, TailModifierNodeProvider};
use crate::foundation::node_coordinator::PerformMeasureHelper;
use crate::foundation::node_coordinator_impl::NodeCoordinatorImpl;
use crate::foundation::placeable::Placeable;

#[derive(Debug, Delegate, AnyConverter)]
pub(crate) struct LayoutModifierNodeCoordinator {
    pub(crate) layout_node: Weak<RefCell<LayoutNode>>,
    pub(crate) layout_modifier_node: Rc<RefCell<dyn ModifierNode>>,
    #[to(
    Placeable,
    Measured,
    DrawableNodeCoordinator,
    NodeCoordinatorTrait,
    MeasureScope,
    PlaceablePlaceAt,
    IntrinsicMeasurable,
    LookaheadCapablePlaceable,
    MeasureResultProvider,
    ParentDataGenerator
    )]
    pub(crate) node_coordinator_impl: NodeCoordinatorImpl,
}

impl TailModifierNodeProvider for LayoutModifierNodeCoordinator {
    fn get_tail(&self) -> Rc<RefCell<dyn ModifierNode>> {
        self.layout_modifier_node.to_delegated_node()
    }
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
    pub(crate) fn new(
        layout_node: &Rc<RefCell<LayoutNode>>,
        measure_node: &Rc<RefCell<dyn ModifierNode>>,
        node_chain: &Rc<RefCell<NodeChain>>,
    ) -> Self {
        let mut result = Self {
            layout_node: Rc::downgrade(layout_node),
            node_coordinator_impl: NodeCoordinatorImpl::new(),
            layout_modifier_node: measure_node.clone(),
        };

        result.node_coordinator_impl.attach(layout_node, node_chain);
        result
    }

    pub(crate) fn set_layout_modifier_node(
        &mut self,
        mut layout_mod: Rc<RefCell<dyn ModifierNode>>,
    ) -> Rc<RefCell<dyn ModifierNode>> {
        std::mem::swap(&mut self.layout_modifier_node, &mut layout_mod);
        layout_mod
    }

    pub(crate) fn on_layout_modifier_node_changed(&self) {
        self.node_coordinator_impl.on_layout_modifier_node_changed()
    }
}

impl Measurable for LayoutModifierNodeCoordinator {
    fn measure(&mut self, constraint: &Constraints) -> (IntSize, Rc<RefCell<dyn Placeable>>) {
        let measure_result = self.perform_measure(constraint, move |this| {
            let node = this.layout_modifier_node.clone();
            let measure_result = if let Some(layout_node_modifier) = node
                .borrow_mut()
                .as_layout_modifier_node_mut()
            {
                let wrapped = this.get_wrapped().unwrap();
                let mut wrapped_not_null = wrapped.borrow_mut();
                let measure_result = layout_node_modifier.measure(
                    this,
                    wrapped_not_null.as_measurable_mut(),
                    constraint,
                );

                measure_result
            } else {
                panic!("downcast from type Node to LayoutNodeModifier failed")
            };

            measure_result
        });
        let size: IntSize = measure_result.as_int_size();

        self.set_measured_result(measure_result);
        self.on_measured();

        (size, self.as_placeable())
    }

    fn as_placeable(&mut self) -> Rc<RefCell<dyn Placeable>> {
        self.node_coordinator_impl.as_placeable()
    }

    fn as_measurable_mut(&mut self) -> &mut dyn Measurable {
        self
    }
}


impl PerformDrawTrait for LayoutModifierNodeCoordinator {}

impl NodeCoordinator for LayoutModifierNodeCoordinator {
    fn as_node_coordinator(&self) -> &dyn NodeCoordinator {
        self
    }
}
