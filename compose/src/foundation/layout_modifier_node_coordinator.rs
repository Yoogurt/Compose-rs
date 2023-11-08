use crate::foundation::constraint::Constraints;
use crate::foundation::layout_modifier_node::LayoutModifierNode;
// use crate::foundation::layout_modifier_node_impl::LayoutModifierNodeImpl;
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::measurable::Measurable;
use crate::foundation::modifier::ModifierNode;
use crate::foundation::node_coordinator::{NodeCoordinator, NodeCoordinatorTrait, PerformDrawTrait};
use crate::foundation::node_coordinator_impl::NodeCoordinatorImpl;
use crate::foundation::placeable::Placeable;
use auto_delegate::Delegate;
use std::any::Any;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::panic::panic_any;
use std::rc::{Rc, Weak};
use compose_foundation_macro::AnyConverter;
use crate::foundation::canvas::Canvas;
use crate::foundation::node_coordinator::PerformMeasureHelper;

#[derive(Debug, Delegate, AnyConverter)]
pub(crate) struct LayoutModifierNodeCoordinator {
    pub(crate) layout_node: Weak<RefCell<LayoutNode>>,
    pub(crate) layout_modifier_node: Rc<RefCell<dyn ModifierNode>>,
    #[to(
    Placeable,
    Measured,
    NodeCoordinatorTrait,
    MeasureScope,
    PlaceablePlaceAt,
    IntrinsicMeasurable,
    LookaheadCapablePlaceable,
    TailModifierNodeProvider,
    MeasureResultProvider
    )]
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
    pub(crate) fn new(
        layout_node: Weak<RefCell<LayoutNode>>,
        measure_node: Rc<RefCell<dyn ModifierNode>>,
    ) -> Self {
        Self {
            layout_node,
            node_coordinator_impl: NodeCoordinatorImpl::new(),
            layout_modifier_node: measure_node,
        }
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
    fn measure(&mut self, constraint: &Constraints) -> Rc<RefCell<dyn Placeable>> {
        self.perform_measure(constraint, move |this| {
            let node = this.layout_modifier_node.clone();
            if let Some(layout_node_modifier) = node
                .borrow_mut()
                .as_layout_modifier_node_mut()
            {
                let wrapped = this.get_wrapped().unwrap();
                let mut wrapped_not_null = wrapped.borrow_mut();
                layout_node_modifier.measure(
                    this,
                    wrapped_not_null.as_measurable_mut(),
                    constraint,
                );
            } else {
                panic!("downcast from type Node to LayoutNodeModifier failed")
            }

            this
        });

        self.on_measured();

        self.as_placeable()
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
    
    fn draw(&self, canvas: &mut dyn Canvas) {
        self.node_coordinator_impl.draw(canvas);
    }
}
