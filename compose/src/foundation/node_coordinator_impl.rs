use std::any::Any;
use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

use auto_delegate::Delegate;
use compose_foundation_macro::{AnyConverter, Leak};

use crate::foundation::canvas::Canvas;
use crate::foundation::composer::Composer;
use crate::foundation::geometry::{IntOffset, IntSize};
use crate::foundation::intrinsic_measurable::IntrinsicMeasurable;
use crate::foundation::layout::layout_coordinates::LayoutCoordinates;
use crate::foundation::look_ahead_capable_placeable::LookaheadCapablePlaceable;
use crate::foundation::look_ahead_capable_placeable_impl::LookaheadCapablePlaceableImpl;
use crate::foundation::measure_result::{MeasureResult, MeasureResultProvider};
use crate::foundation::measure_scope::MeasureScope;
use crate::foundation::memory::leak_token::LeakToken;
use crate::foundation::modifier::{ModifierNode, ModifierNodeExtension, NodeKind};
use crate::foundation::node::LayoutNodeDrawScope;
use crate::foundation::node_chain::{NodeChain, TailModifierNode};
use crate::foundation::node_coordinator::{DrawableNodeCoordinator, NodeCoordinatorTrait, PerformDrawTrait};
use crate::foundation::node_coordinator::TailModifierNodeProvider;
use crate::foundation::parent_data::ParentDataGenerator;
use crate::foundation::placeable_place_at::PlaceablePlaceAt;
use crate::foundation::ui::draw::{CanvasDrawScope, DrawContext};
use crate::foundation::utils::box_wrapper::WrapWithBox;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;
use crate::foundation::utils::weak_upgrade::WeakUpdater;

use super::constraint::Constraints;
use super::layout_node::LayoutNode;
use super::measurable::Measurable;
use super::node_coordinator::NodeCoordinator;
use super::placeable::Placeable;

#[Leak]
#[derive(Debug, Delegate, AnyConverter)]
pub(crate) struct NodeCoordinatorImpl {
    #[to(Placeable, Measured, MeasureScope, LookaheadCapablePlaceable)]
    pub(crate) look_ahead_capable_placeable_impl: LookaheadCapablePlaceableImpl,
    pub(crate) wrapped: Option<Rc<RefCell<dyn NodeCoordinator>>>,
    pub(crate) wrapped_by: Option<Weak<RefCell<dyn NodeCoordinator>>>,
    pub(crate) layout_node: Weak<RefCell<LayoutNode>>,
    pub(crate) node_chain: Weak<RefCell<NodeChain>>,

    pub(crate) z_index: f32,

    pub(crate) tail: Rc<RefCell<dyn ModifierNode>>,

    pub(crate) measure_result: Option<MeasureResult>,

    perform_draw_vtable: Option<Weak<RefCell<dyn PerformDrawTrait>>>,
}

impl ParentDataGenerator for NodeCoordinatorImpl {
    fn generate_parent_data(&self) -> Option<Box<dyn Any>> {
        let mut data = None;
        let density = self.get_density();

        self.node_chain.upgrade().unwrap().borrow_mut().tail_to_head(|node| {
            if node.get_node_kind() == NodeKind::ParentData {
                node.dispatch_for_kind_mut(NodeKind::ParentData, |it| {
                    data = (it.as_parent_data_modifier_node_mut().unwrap().modify_parent_data(density, data.take()));
                });
            }
        });

        data
    }
}

impl IntrinsicMeasurable for NodeCoordinatorImpl {
    fn get_parent_data(&self) -> Option<&dyn Any> {
        unimplemented!()
    }
}

impl Measurable for NodeCoordinatorImpl {
    fn measure(&mut self, _constraint: &Constraints) -> (IntSize, Rc<RefCell<dyn Placeable>>) {
        unimplemented!("layout node wrapper should implement measure")
    }

    fn as_placeable(&mut self) -> Rc<RefCell<dyn Placeable>> {
        self.look_ahead_capable_placeable_impl.as_placeable()
    }

    fn as_measurable_mut(&mut self) -> &mut dyn Measurable {
        unimplemented!("layout node wrapper should implement as_measurable_mut")
    }
}

impl NodeCoordinatorImpl {
    pub(crate) fn attach(&mut self, layout_node: &Rc<RefCell<LayoutNode>>, node_chain: &Rc<RefCell<NodeChain>>) {
        self.layout_node = Rc::downgrade(layout_node);
        self.node_chain = Rc::downgrade(node_chain);
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

impl PerformDrawTrait for NodeCoordinatorImpl {
    fn perform_draw(&self, canvas: &mut dyn Canvas) {
        match self.perform_draw_vtable.as_ref() {
            Some(perform_draw_trait) => {
                if let Some(vtable) = perform_draw_trait.upgrade() {
                    vtable.borrow().perform_draw(canvas);
                }
            }
            None => {
                if let Some(wrapped) = self.get_wrapped().as_ref() {
                    wrapped.borrow().draw(canvas);
                }
            }
        }
    }
}

impl MeasureResultProvider for NodeCoordinatorImpl {
    fn set_measured_result(&mut self, measure_result: MeasureResult) {
        match self.measure_result.as_ref() {
            Some(old_measure_result) => {
                if old_measure_result == &measure_result {
                    return;
                }
            }
            None => {}
        }

        let size = measure_result.as_int_size();
        self.measure_result = Some(measure_result);

        self.on_measure_result_changed(size)
    }

    fn get_measured_result(&mut self) -> Option<MeasureResult> {
        self.measure_result.take()
    }

    fn has_measure_result(&self) -> bool {
        self.measure_result.is_some()
    }
}

impl LayoutCoordinates for NodeCoordinatorImpl {
    fn size(&self) -> IntSize {
        self.get_measured_size()
    }

    fn is_attached(&self) -> bool {
       self.layout_node.upgrade().map(|layout_node| layout_node.borrow().is_attached()).unwrap_or(false)
    }
}

impl NodeCoordinator for NodeCoordinatorImpl {
    fn as_node_coordinator(&self) -> &dyn NodeCoordinator {
        self
    }

    fn on_placed(&self) {
        self.visit_nodes(NodeKind::LayoutAware, |modifier_node| {
            modifier_node.borrow().as_layout_aware_modifier_node().unwrap().on_placed(self);
        });
    }
}

impl DrawableNodeCoordinator for NodeCoordinatorImpl {
    fn draw(&self, canvas: &mut dyn Canvas) {
        let offset = self.get_position().as_f32_offset();
        canvas.translate(offset.x, offset.y);
        self.draw_contrained_draw_modifiers(canvas);
        canvas.translate(-offset.x, -offset.y);
    }
}

impl TailModifierNodeProvider for NodeCoordinatorImpl {
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
            node_chain: Weak::new(),
            z_index: 0.0,
            tail: TailModifierNode::default().wrap_with_rc_refcell(),

            measure_result: None,
            perform_draw_vtable: None,
            leak_object: Default::default()
        }
    }

    pub fn set_vtable_perform_draw_trait(&mut self, perform_draw_vtable: Weak<RefCell<dyn PerformDrawTrait>>) {
        self.perform_draw_vtable = Some(perform_draw_vtable);
    }

    pub fn set_vtable_placeable_place_at(&mut self, place_at_vtable: Weak<RefCell<dyn PlaceablePlaceAt>>) {
        self.look_ahead_capable_placeable_impl.placeable_impl.borrow_mut().set_vtable(place_at_vtable);
    }

    pub(crate) fn on_layout_modifier_node_changed(&self) {}

    fn place_self(&mut self, position: IntOffset, z_index: f32) {
        if self.get_position() != position {
            self.set_position(position);
        }

        self.z_index = z_index;
    }

    fn head_node(&self, include_tail: bool) -> Option<Rc<RefCell<dyn ModifierNode>>> {
        let node_chain = self.layout_node().upgrade().unwrap().borrow().node_chain.clone();

        if node_chain.borrow().outer_coordinator.as_ptr() as *const () == self as *const NodeCoordinatorImpl as *const () {
            Some(node_chain.borrow().head.clone())
        } else {
            self.get_wrapped_by().and_then(|wrapped_by| {
                let tail = wrapped_by.borrow().get_tail();
                if include_tail {
                    dbg!(self);
                    dbg!("wrapped_by", &wrapped_by);
                    dbg!("wrapped_by child", tail.borrow().get_child());
                    tail.borrow().get_child()
                } else {
                    Some(tail)
                }
            })
        }
    }

    fn visit_nodes(&self, mask: impl Into<u32>, mut block: impl FnMut(&Rc<RefCell<dyn ModifierNode>>)) {
        let mut stop_node = self.get_tail();
        let mask = mask.into();
        let include_tail = mask & NodeKind::LayoutAware as u32 != 0;

        if !include_tail {
            let node = match stop_node.borrow().get_parent() {
                Some(parent) => { parent }
                None => { return; }
            };
            stop_node = node;
        };

        let mut node = self.head_node(include_tail);

        while let Some(visit) = node {
            if visit.borrow().get_node_kind() as u32 & mask != 0 {
                block(&visit);
            }

            if visit.as_ptr() == stop_node.as_ptr() {
                return;
            }

            node = visit.borrow().get_child();
        }
    }

    fn head(&self, node_kind: NodeKind) -> Option<Rc<RefCell<dyn ModifierNode>>> {
        let mut stop_node = self.get_tail();
        let include_tail = (node_kind as u32 & NodeKind::LayoutAware as u32) != 0;
        if !include_tail {
            let node = match stop_node.borrow().get_parent() {
                Some(parent) => { parent }
                None => { return None; }
            };
            stop_node = node;
        };

        let mut node = self.head_node(include_tail);

        while let Some(visit) = node {
            if visit.borrow().get_node_kind() == node_kind {
                return Some(visit);
            }

            if visit.as_ptr() as *const () == stop_node.as_ptr() as *const () {
                return None;
            }

            node = visit.borrow().get_child();
        }

        None
    }

    fn draw_contrained_draw_modifiers(&self, canvas: &mut dyn Canvas) {
        // dbg!(self);
        let head = self.head(NodeKind::Draw);

        match head {
            Some(head) => {
                // new instance of layout draw scope
                // todo use share instead
                let density = self.get_density();
                let draw_context = DrawContext::new(self.get_size().as_f32_size(), density, canvas);

                let layout_direction = self.get_layout_direction();
                let canvas_draw_scope = CanvasDrawScope::new(draw_context, layout_direction);
                let draw_scope = LayoutNodeDrawScope::new(canvas_draw_scope).wrap_with_box();

                draw_scope.draw(head)
            }
            None => {
                self.perform_draw(canvas)
            }
        }
    }

    fn on_measure_result_changed(&mut self, size: IntSize) {
        self.set_measured_size(size);
        // self.visit_nodes(NodeKind::Draw, true, |draw_modifier_node| {
        //     draw_modifier_node.borrow_mut().as_draw_modifier_node_mut().unwrap().on_measure_result_changed();
        // });
    }
}

impl PlaceablePlaceAt for NodeCoordinatorImpl {
    fn place_at(&mut self, position: IntOffset, z_index: f32) {
        self.place_self(position, z_index)
    }
}
