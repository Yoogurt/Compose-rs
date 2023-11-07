use super::constraint::Constraints;
use super::layout_node::LayoutNode;
use super::measurable::Measurable;
use super::node_coordinator::NodeCoordinator;
use super::placeable::Placeable;
use crate::foundation::geometry::{IntOffset, IntSize};
use crate::foundation::intrinsic_measurable::IntrinsicMeasurable;
use crate::foundation::look_ahead_capable_placeable::LookaheadCapablePlaceable;
use crate::foundation::look_ahead_capable_placeable_impl::LookaheadCapablePlaceableImpl;
use crate::foundation::node_coordinator::{NodeCoordinatorTrait, PerformDrawTrait};
use crate::foundation::placeable_place_at::PlaceablePlaceAt;
use crate::foundation::utils::weak_upgrade::WeakUpdater;
use auto_delegate::Delegate;
use std::any::Any;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
use compose_foundation_macro::AnyConverter;
use crate::foundation::canvas::Canvas;
use crate::foundation::modifier::{ModifierNode, NodeKind};
use crate::foundation::node::LayoutNodeDrawScope;
use crate::foundation::node_chain::TailModifierNode;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;
use crate::foundation::node_coordinator::TailModifierNodeProvider;
use crate::foundation::ui::draw::{CanvasDrawScope, ContentDrawScope, DrawContext};
use crate::foundation::utils::box_wrapper::WrapWithBox;

#[derive(Debug, Delegate, AnyConverter)]
pub(crate) struct NodeCoordinatorImpl {
    #[to(Placeable, Measured, MeasureScope, LookaheadCapablePlaceable)]
    pub(crate) look_ahead_capable_placeable_impl: LookaheadCapablePlaceableImpl,
    pub(crate) wrapped: Option<Rc<RefCell<dyn NodeCoordinator>>>,
    pub(crate) wrapped_by: Option<Weak<RefCell<dyn NodeCoordinator>>>,
    pub(crate) layout_node: Weak<RefCell<LayoutNode>>,
    pub(crate) z_index: f32,

    pub(crate) tail: Rc<RefCell<dyn ModifierNode>>,
    pub(crate) parent_data: Option<Box<dyn Any>>,
}

impl IntrinsicMeasurable for NodeCoordinatorImpl {
    fn set_parent_data(&mut self, parent_data: Option<Box<dyn Any>>) {
        self.parent_data = parent_data;
    }

    fn get_parent_data(&self) -> Option<&Box<dyn Any>> {
        self.parent_data.as_ref()
    }

    fn get_parent_data_mut(&mut self) -> Option<&mut Box<dyn Any>> {
        self.parent_data.as_mut()
    }
}

impl Measurable for NodeCoordinatorImpl {
    fn measure(&mut self, _constraint: &Constraints) -> &mut dyn Placeable {
        unimplemented!("layout node wrapper should implement measure")
    }

    fn as_placeable_mut(&mut self) -> &mut dyn Placeable {
        unimplemented!("layout node wrapper should implement as_placeable_mut")
    }

    fn as_measurable_mut(&mut self) -> &mut dyn Measurable {
        unimplemented!("layout node wrapper should implement as_measurable_mut")
    }
}

impl NodeCoordinatorImpl {
    pub(crate) fn attach(&mut self, layout_node: Weak<RefCell<LayoutNode>>) {
        self.layout_node = Weak::new();
    }

    pub(crate) fn layout_node(&self) -> Weak<RefCell<LayoutNode>> {
        self.layout_node.clone()
    }
}

impl DerefMut for NodeCoordinatorImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.look_ahead_capable_placeable_impl
    }
}

impl Deref for NodeCoordinatorImpl {
    type Target = dyn LookaheadCapablePlaceable;
    fn deref(&self) -> &Self::Target {
        &self.look_ahead_capable_placeable_impl
    }
}

impl NodeCoordinatorTrait for NodeCoordinatorImpl {
    fn set_wrapped(&mut self, wrapped: Option<Rc<RefCell<dyn NodeCoordinator>>>) {
        self.wrapped = wrapped
    }

    fn get_wrapped(&self) -> Option<Rc<RefCell<dyn NodeCoordinator>>> {
        self.wrapped.clone()
    }

    fn set_wrapped_by(&mut self, wrapped_by: Option<Weak<RefCell<dyn NodeCoordinator>>>) {
        self.wrapped_by = wrapped_by;
    }

    fn get_wrapped_by(&self) -> Option<Rc<RefCell<dyn NodeCoordinator>>> {
        self.wrapped_by.try_upgrade()
    }

    fn get_z_index(&self) -> f32 {
        self.z_index
    }

    fn set_z_index(&mut self, z_index: f32) {
        self.z_index = z_index;
    }
}

impl PerformDrawTrait for NodeCoordinatorImpl {}

impl NodeCoordinator for NodeCoordinatorImpl {
    fn draw(&mut self, canvas: &mut dyn Canvas) {
        let offset = self.get_position().as_f32_offset();
        canvas.translate(offset.x(), offset.y());
        self.draw_contrained_draw_modifiers(canvas);
        canvas.translate(-offset.x(), -offset.y());
    }
}

impl TailModifierNodeProvider for NodeCoordinatorImpl {
    fn set_tail(&mut self, tail: Rc<RefCell<dyn ModifierNode>>) {
        self.tail = tail;
    }

    fn get_tail(&self) -> Rc<RefCell<dyn ModifierNode>> {
        self.tail.clone()
    }
}

impl NodeCoordinatorImpl {
    pub(crate) fn new() -> Self {
        NodeCoordinatorImpl {
            look_ahead_capable_placeable_impl: LookaheadCapablePlaceableImpl::default(),
            wrapped: None,
            wrapped_by: None,
            layout_node: Weak::new(),
            parent_data: None,
            z_index: 0.0,
            tail: TailModifierNode::default().wrap_with_rc_refcell(),
        }
    }

    pub(crate) fn on_layout_modifier_node_changed(&self) {}

    fn place_self(&mut self, position: IntOffset, z_index: f32) {
        if self.get_position() != position {
            self.set_position(position);
        }

        self.z_index = z_index;
    }

    fn head_node(&self, include_tail: bool) -> Option<Rc<RefCell<dyn ModifierNode>>> {
        let node_chain = self.layout_node.upgrade().unwrap().borrow().node_chain.clone();


        if std::ptr::eq(node_chain.borrow().outer_coordinator.as_ptr(), self) {
            Some(node_chain.borrow().head.clone())
        } else {
            self.get_wrapped_by().and_then(|wrapped_by| {
                let tail = wrapped_by.borrow().get_tail();
                if include_tail {
                    tail.borrow().get_child()
                } else {
                    Some(tail)
                }
            })
        }
    }

    fn head(&self, node_kind: NodeKind, include_tail: bool) -> Option<Rc<RefCell<dyn ModifierNode>>> {
        let mut stop_node = self.get_tail();
        if !include_tail {
            let node = match stop_node.borrow().get_parent() {
                Some(parent) => { parent }
                None => { return None; }
            };
            stop_node = node;
        };

        let mut node = self.head_node(include_tail);

        while let Some(visit) = node {
            if visit.borrow_mut().get_node_kind() == node_kind {
                return Some(visit);
            }

            node = visit.borrow().get_child();
        }

        None
    }

    fn draw_contrained_draw_modifiers(&mut self, canvas: &mut dyn Canvas) {
        let head = self.head(NodeKind::DrawModifierNode, false);

        match head {
            Some(head) => {
                // new instance of layout draw scope
                // todo use share instead
                let density = self.get_density();
                let draw_context = DrawContext::new(self.get_measured_size().as_f32_size(), density, canvas);

                let layout_direction = self.get_layout_direction();
                let canvas_draw_scope = CanvasDrawScope::new(draw_context, layout_direction);
                let draw_scope = LayoutNodeDrawScope::new(canvas_draw_scope).wrap_with_box();

                // draw_scope.draw()
            }
            None => {
                self.perform_draw(canvas)
            }
        }
    }
}

impl PlaceablePlaceAt for NodeCoordinatorImpl {
    fn place_at(&mut self, position: IntOffset, z_index: f32) {
        self.place_self(position, z_index)
    }
}
