#![feature(trait_upcasting)]

use std::any::Any;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

use auto_delegate::Delegate;
use compose_foundation_macro::{AnyConverter, Leak};
use crate::foundation::geometry::Size;
use crate::foundation::canvas::Canvas;
use crate::foundation::composer::Composer;
use crate::foundation::geometry::{Density, IntOffset, IntSize, Offset};
use crate::foundation::intrinsic_measurable::IntrinsicMeasurable;
use crate::foundation::layout::layout_coordinates::LayoutCoordinates;
use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::look_ahead_capable_placeable::LookaheadCapablePlaceable;
use crate::foundation::look_ahead_capable_placeable_impl::LookaheadCapablePlaceableImpl;
use crate::foundation::measure_result::{MeasureResult, MeasureResultProvider};
use crate::foundation::measure_scope::MeasureScope;
use crate::foundation::measured::Measured;
use crate::foundation::memory::leak_token::LeakToken;
use crate::foundation::modifier::{ModifierNode, ModifierNodeExtension, NodeKind};
use crate::foundation::node::{LayoutNodeDrawScope, OwnedLayer, SkiaOwnedLayer};
use crate::foundation::node_chain::{NodeChain, TailModifierNode};
use crate::foundation::node_coordinator::{AsNodeCoodinator, DrawableNodeCoordinator, HitTestSource, HitTestTrait, NodeCoordinatorTrait, PerformDrawTrait};
use crate::foundation::node_coordinator::TailModifierNodeProvider;
use crate::foundation::parent_data::ParentDataGenerator;
use crate::foundation::placeable_place_at::PlaceablePlaceAt;
use crate::foundation::ui::draw::{CanvasDrawScope, DrawContext};
use crate::foundation::ui::graphics::graphics_layer_modifier::GraphicsLayerScope;
use crate::foundation::ui::hit_test_result::HitTestResult;
use crate::foundation::utils::box_wrapper::WrapWithBox;
use crate::foundation::utils::option_extension::OptionThen;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;
use crate::foundation::utils::weak_upgrade::WeakUpdater;

use super::constraint::Constraints;
use super::layout_node::LayoutNode;
use super::measurable::Measurable;
use super::node_coordinator::NodeCoordinator;
use super::placeable::Placeable;

#[Leak]
#[derive(Delegate, AnyConverter)]
pub struct NodeCoordinatorImpl {
    #[to(Placeable, MeasureScope, LookaheadCapablePlaceable)]
    pub(crate) look_ahead_capable_placeable_impl: LookaheadCapablePlaceableImpl,
    pub(crate) wrapped: Option<Rc<RefCell<dyn NodeCoordinator>>>,
    pub(crate) wrapped_by: Option<Weak<RefCell<dyn NodeCoordinator>>>,
    pub(crate) layout_node: Weak<RefCell<LayoutNode>>,
    pub(crate) node_chain: Weak<RefCell<NodeChain>>,

    pub(crate) z_index: f32,

    pub(crate) tail: Rc<RefCell<dyn ModifierNode>>,

    pub(crate) measure_result: Option<MeasureResult>,

    is_clipping: bool,

    layer: Option<Box<dyn OwnedLayer>>,
    layer_block: Option<Rc<dyn Fn(&mut GraphicsLayerScope)>>,
    layer_density: Density,
    layer_layout_direction: LayoutDirection,

    graphics_layer_scope: GraphicsLayerScope,

    vtable_raw: Option<*mut dyn NodeCoordinator>,
}

impl Measured for NodeCoordinatorImpl {
    fn get_measured_width(&self) -> usize {
        self.get_measured_size().width
    }

    fn get_measured_height(&self) -> usize {
        self.get_measured_size().height
    }
}

impl Debug for NodeCoordinatorImpl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeCoordinatorImpl").field("wrapped", &self.wrapped)
            .field("wrapped_by", &self.wrapped_by)
            .field("tail", &self.tail)
            .field("measure_result", &self.measure_result)
            .finish()
    }
}

impl ParentDataGenerator for NodeCoordinatorImpl {
    fn generate_parent_data(&self) -> Option<Box<dyn Any>> {
        let mut data = None;
        let density = self.get_density();

        self.node_chain.upgrade().unwrap().borrow_mut().tail_to_head(|node| {
            if node.is_node_kind(NodeKind::ParentData) {
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
    pub(crate) const PointerInputSource: PointerInputSource = PointerInputSource;

    pub(crate) fn attach(&mut self, layout_node: &Rc<RefCell<LayoutNode>>, node_chain: &Rc<RefCell<NodeChain>>) {
        self.layout_node = Rc::downgrade(layout_node);
        self.node_chain = Rc::downgrade(node_chain);
    }

    pub(crate) fn layout_node(&self) -> Weak<RefCell<LayoutNode>> {
        self.layout_node.clone()
    }

    pub(crate) fn within_layer_bounds(&self, position: Offset<f32>) -> bool {
        if !position.is_finite() {
            return false;
        }

        let layer = self.layer.as_ref();
        self.is_clipping || layer.map(|layer| layer.is_in_layer(position)).unwrap_or(true)
    }

    fn is_pointer_in_bounds(&self, pointer_position: Offset<f32>) -> bool {
        let x = pointer_position.x;
        let y = pointer_position.y;

        x >= 0f32 && y >= 0f32 && x < self.get_measured_width() as f32 && y < self.get_measured_height() as f32
    }

    fn hit(&self, this: Option<Rc<RefCell<dyn ModifierNode>>>, hit_test_source: &dyn HitTestSource, pointer_position: Offset<f32>, hit_test_result: &mut HitTestResult, is_touch_event: bool, is_in_layer: bool) {
        match this {
            None => {
                self.hit_test_child(hit_test_source, pointer_position, hit_test_result, is_touch_event, is_in_layer)
            }
            Some(this) => {
                hit_test_result.hit(&this, is_in_layer, |hit_test_result| {
                    let node = Self::next_until(this.clone(), hit_test_source.entity_type(), NodeKind::Layout);
                    self.hit(node, hit_test_source, pointer_position, hit_test_result, is_touch_event, is_in_layer)
                });
            }
        }
    }

    fn next_until(node: Rc<RefCell<dyn ModifierNode>>, node_kind: NodeKind, stop_type: NodeKind) -> Option<Rc<RefCell<dyn ModifierNode>>> {
        let child = node.borrow().get_child();
        let Some(child) = child else {
            return None;
        };

        if child.borrow().get_aggregate_child_kind_set() & node_kind.mask() == 0 {
            return None;
        }

        let mut next = Some(child.clone());
        while let Some(ref next_ref) = next {
            let next_ref = next_ref.borrow();
            let kind_set = next_ref.get_kind_set();
            if kind_set & stop_type.mask() != 0 {
                return None
            }

            if node_kind & kind_set != 0  {
                drop(next_ref);
                return next
            }

            let next_child = next_ref.get_child();
            drop(next_ref);
            next = next_child;
        }

        None
    }

    fn hit_near(&self, node: Option<Rc<RefCell<dyn ModifierNode>>>, hit_test_source: &dyn HitTestSource, pointer_position: Offset<f32>, hit_test_result: &mut HitTestResult, is_touch_event: bool, is_in_layer: bool, distance_from_edge: f32) {
        match node {
            None => {
                // self.hit_test_child(hit_test_source, hit_test_result, pointer_position, is_touch_event, is_in_layer)
            }
            Some(node) => {}
        }
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
        unsafe {
            match self.vtable_raw.map(|vtable| vtable.as_ref()) {
                Some(perform_draw_trait) => {
                    if let Some(vtable) = perform_draw_trait {
                        vtable.perform_draw(canvas);
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

    fn get_parent_coordinates(&self) -> Option<Rc<RefCell<dyn LayoutCoordinates>>> {
        self.get_wrapped_by().map(|wrapped_by| wrapped_by as Rc<RefCell<dyn LayoutCoordinates>>)
    }

    fn get_parent_layout_coordinates(&self) -> Option<Rc<RefCell<dyn LayoutCoordinates>>> {
        self.layout_node().upgrade().and_then(|layout_node| layout_node.borrow().get_outer_coordinator().borrow().get_wrapped_by())
            .map(|wrapped_by| wrapped_by as Rc<RefCell<dyn LayoutCoordinates>>)
    }
}

impl AsNodeCoodinator for NodeCoordinatorImpl {
    fn node_coordinator_ref(&self) -> &NodeCoordinatorImpl {
        &self
    }
}

impl NodeCoordinator for NodeCoordinatorImpl {
    fn on_placed(&self) {
        self.visit_nodes(NodeKind::LayoutAware, |modifier_node| {
            modifier_node.borrow().as_layout_aware_modifier_node().unwrap().on_placed(self);
        });
    }

    fn get_layer(&self) -> Option<&Box<dyn OwnedLayer>> {
        self.layer.as_ref()
    }
}

impl HitTestTrait for NodeCoordinatorImpl {
    fn hit_test(&self,
                hit_test_source: &dyn HitTestSource,
                pointer_position: Offset<f32>,
                hit_test_result: &mut HitTestResult,
                is_touch_event: bool,
                is_in_layer: bool) {
        let head = self.head(hit_test_source.entity_type());

        if !self.within_layer_bounds(pointer_position) {
            let density;
            let minimum_touch_target_size;
            {
                let layout_node = self.layout_node.upgrade().unwrap();
                let layout_node_ref = layout_node.borrow();
                density = layout_node_ref.get_density();
                minimum_touch_target_size = layout_node_ref.view_configuration.minimumTouchTargetSize.to_size(density);
            }

            let distance_from_edge = self.distance_in_minimum_touch_target(pointer_position, minimum_touch_target_size);

            if distance_from_edge.is_finite()
                && hit_test_result.is_hit_in_minimum_touch_target_better(distance_from_edge, false) {
                
            }
        } else if head.is_none() {
            self.hit_test_child(hit_test_source, pointer_position, hit_test_result, is_touch_event, is_in_layer)
        } else if self.is_pointer_in_bounds(pointer_position) {
            self.hit(head, hit_test_source, pointer_position, hit_test_result, is_touch_event, is_in_layer);
        } else {
        }
    }

    fn should_share_pointer_input_with_siblings(&self) -> bool {
        // self.head_node(NodeKind::PointerInput).map_or(false, |start| {
        //     false
        // })
        false
    }

    fn hit_test_child(&self, hit_test_source: &dyn HitTestSource, pointer_position: Offset<f32>, hit_test_result: &mut HitTestResult, is_touch_event: bool, is_in_layer: bool) {
        self.vtable_raw.as_ref().then(|vtable| unsafe {
            vtable.as_ref().unwrap().hit_test_child(hit_test_source, pointer_position, hit_test_result, is_touch_event, is_in_layer)
        });
    }
}

impl DrawableNodeCoordinator for NodeCoordinatorImpl {
    fn draw(&self, canvas: &mut dyn Canvas) {
        if let Some(layer) = self.layer.as_ref() {
            layer.draw_layer(canvas)
        } else {
            let offset = self.get_position().as_f32_offset();
            canvas.translate(offset.x, offset.y);
            self.draw_contrained_draw_modifiers(canvas);
            canvas.translate(-offset.x, -offset.y);
        }
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

            graphics_layer_scope: GraphicsLayerScope::new(),
            measure_result: None,
            leak_object: Default::default(),

            is_clipping: false,
            layer_block: None,
            layer_density: Density::default(),
            layer_layout_direction: LayoutDirection::default(),
            layer: None,

            vtable_raw: None,
        }
    }

    pub(crate) fn set_vtable(&mut self, vtable: *const dyn NodeCoordinator) {
        self.vtable_raw = Some(vtable as *const dyn NodeCoordinator as *mut dyn NodeCoordinator);
    }

    pub(crate) fn set_vtable_placeable_place_at(&mut self, place_at_vtable: Weak<RefCell<dyn PlaceablePlaceAt>>) {
        self.look_ahead_capable_placeable_impl.placeable_impl.borrow_mut().set_vtable(place_at_vtable);
    }

    pub(crate) fn on_layout_modifier_node_changed(&self) {}

    fn compare_layer_block(left: Option<&Rc<dyn Fn(&mut GraphicsLayerScope)>>, right: Option<&Rc<dyn Fn(&mut GraphicsLayerScope)>>) -> bool {
        match (left, right) {
            (Some(left), Some(right)) => {
                Rc::as_ptr(left) == Rc::as_ptr(right)
            }
            _ => { false }
        }
    }

    fn update_layer_parameters(&mut self, size: IntSize) {
        let layer_density = self.layer_density;

        match self.layer.as_mut() {
            Some(layer) => {
                let layer_block = self.layer_block.clone().unwrap();

                let graphics_layer_scope = &mut self.graphics_layer_scope;

                graphics_layer_scope.reset();
                graphics_layer_scope.set_density(layer_density);
                graphics_layer_scope.set_size(size.as_f32_size());

                layer_block(graphics_layer_scope);

                layer.update_layer_property(
                    graphics_layer_scope
                )
            }
            None => {
                if self.layer_block.is_some() {
                    panic!("self.layer_block should be None")
                }
            }
        }
    }

    pub(crate) fn distance_in_minimum_touch_target(
        &self, pointer_position: Offset<f32>, minium_touch_target_size: Size<f32>,
    ) -> f32 {
        if self.get_measured_width() as f32 >= minium_touch_target_size.width
            && self.get_measured_height() as f32 >= minium_touch_target_size.height {
            return f32::MAX;
        }

        0f32
    }

    pub(crate) fn update_layer_block(&mut self, size: IntSize, layer_block: Option<Rc<dyn Fn(&mut GraphicsLayerScope)>>, force_update_parameters: bool) {
        let layout_node = self.layout_node.upgrade().unwrap();
        let layout_node_density = layout_node.borrow().get_density();
        let layout_node_layout_direction = layout_node.borrow().get_layout_direction();

        let update_parameters = force_update_parameters
            || Self::compare_layer_block(self.layer_block.as_ref(), layer_block.as_ref())
            || (self.layer_density != layout_node_density)
            || (self.layer_layout_direction != layout_node_layout_direction);

        self.layer_block = layer_block.clone();
        self.layer_density = layout_node_density;
        self.layer_layout_direction = layout_node_layout_direction;

        if self.is_attached() && layer_block.is_some() {
            if self.layer.is_none() {
                let vtable = self.vtable_raw.clone();

                self.layer = {
                    let layout_node = self.layout_node.clone();

                    let layer: Box<dyn OwnedLayer> = Box::new(SkiaOwnedLayer::new(move |canvas| unsafe {
                        let Some(layout_node) = layout_node.upgrade() else {
                            return;
                        };
                        if layout_node.borrow().is_placed() {
                            let vtable = vtable.as_ref().unwrap().as_ref().unwrap();
                            vtable.node_coordinator_ref().draw_contrained_draw_modifiers(canvas);
                        }
                    }));

                    Some(layer)
                };

                self.update_layer_parameters(size);
            } else if update_parameters {
                todo!()
            }
        } else {
            self.layer = None;
        }
    }

    pub(crate) fn place_self(&mut self, position: IntOffset, size: IntSize, z_index: f32, layer_block: Option<Rc<dyn Fn(&mut GraphicsLayerScope)>>) {
        self.update_layer_block(size, layer_block, false);

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
            if visit.borrow().is_node_kind(node_kind) {
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
    fn place_at(&mut self, position: IntOffset, _size: IntSize, z_index: f32, layer_block: Option<Rc<dyn Fn(&mut GraphicsLayerScope)>>) {
        todo!()
    }
}

pub(crate) struct PointerInputSource;

impl HitTestSource for PointerInputSource {
    fn entity_type(&self) -> NodeKind {
        NodeKind::PointerInput
    }

    fn intercept_out_of_bounds_child_events(&self, node: Rc<RefCell<dyn ModifierNode>>) -> bool {
        node.borrow().dispatch_for_kind(NodeKind::PointerInput, |it| {
            // it.as_pointer_input_modifier_node().unwrap().intercept_out_of_bounds_child_events()
        });

        true
    }

    fn should_hit_test_children(&self, parnet_layout_node: Rc<RefCell<LayoutNode>>) -> bool {
        true
    }

    fn child_hit_test(&self, layout_node: Rc<RefCell<LayoutNode>>, pointer_position: Offset<f32>, hit_test_result: &mut HitTestResult, is_touch_event: bool, is_in_layer: bool) {
        let hit_test_delegate = layout_node.borrow().layout_node_hit_test_delegate.clone();
        hit_test_delegate.borrow().hit_test(pointer_position, hit_test_result, is_touch_event, is_in_layer);
    }
}