use crate::foundation::constraint::Constraints;
use crate::foundation::geometry::{IntOffset, IntSize};
use crate::foundation::intrinsic_measurable::IntrinsicMeasurable;
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::layout_state::LayoutState;
use crate::foundation::measurable::Measurable;
use crate::foundation::node_chain::NodeChain;
use crate::foundation::node_coordinator::NodeCoordinator;
use crate::foundation::placeable::Placeable;
use crate::foundation::placeable_impl::PlaceableImpl;
use crate::foundation::placeable_place_at::PlaceablePlaceAt;
use crate::foundation::remeasurable::{Remeasurable, StatefulRemeasurable};
use crate::foundation::usage_by_parent::UsageByParent;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;
use auto_delegate::Delegate;
use std::any::Any;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use crate::foundation::inner_node_coordinator::InnerNodeCoordinator;
use crate::foundation::utils::self_reference::SelfReference;

#[derive(Debug, Delegate)]
pub(crate) struct MeasurePassDelegate {
    pub(crate) placeable_impl: Rc<RefCell<PlaceableImpl>>,
    pub(crate) nodes: Option<Rc<RefCell<NodeChain>>>,
    pub(crate) remeasure_pending: bool,
    pub(crate) measure_pending: bool,
    pub(crate) layout_pending: bool,
    pub(crate) measured_by_parent: UsageByParent,
    pub(crate) last_position: IntOffset,
    pub(crate) last_z_index: f32,
    pub(crate) z_index: f32,
    pub(crate) place_once: bool,
    pub(crate) is_placed: bool,
    pub(crate) layout_state: Option<Rc<RefCell<LayoutState>>>,
    pub(crate) parent_data: Option<Box<dyn Any>>,
    weak_self: Weak<RefCell<Self>>,
    laying_out_children: bool,

    identify: u32,
}

impl Remeasurable for MeasurePassDelegate {
    fn remeasure(&mut self, constraint: &Constraints) -> bool {
        let mut placeable = self.placeable_impl.borrow_mut();

        if !self.measure_pending && placeable.get_measurement_constraint() == *constraint {
            return false;
        }

        let previous_size: IntSize = {
            let outer_coordinator = self.get_outer_coordinator();

            let outer_coordinator = outer_coordinator.borrow_mut();
            outer_coordinator.get_measured_size()
        };

        placeable.set_measurement_constraint(constraint);
        drop(placeable);

        self.perform_measure(constraint);

        let mut placeable = self.placeable_impl.borrow_mut();
        let new_size = {
            let outer_node_ref = unsafe { self.nodes.as_ref().unwrap().borrow() };
            let outer_coordinator = outer_node_ref.outer_coordinator.borrow_mut();
            outer_coordinator.get_measured_size()
        };

        let size = placeable.get_size();

        let size_changed = previous_size != new_size
            || size.width() != new_size.width()
            || size.height() != new_size.height();

        placeable.set_measured_size(new_size);
        size_changed
    }
}

impl StatefulRemeasurable for MeasurePassDelegate {
    fn mark_remeasure_pending(&mut self) {
        self.remeasure_pending = true;
    }
}

impl PlaceablePlaceAt for MeasurePassDelegate {
    fn place_at(&mut self, position: IntOffset, z_index: f32) {
        if position != self.last_position {
            self.last_position = position;
        }

        self.place_outer_coordinator(position, z_index);
    }
}

impl MeasurePassDelegate {
    pub(crate) fn new() -> Rc<RefCell<Self>> {
        let mut result = MeasurePassDelegate {
            placeable_impl: PlaceableImpl::new().wrap_with_rc_refcell(),
            nodes: None,
            remeasure_pending: false,
            measure_pending: false,
            layout_pending: false,
            measured_by_parent: UsageByParent::NotUsed,
            last_position: IntOffset::new(0, 0),
            last_z_index: 0f32,
            z_index: 0f32,
            place_once: false,
            is_placed: false,
            layout_state: None,
            parent_data: None,
            laying_out_children: false,
            identify: 0,
            weak_self: Weak::default(),
        }.wrap_with_rc_refcell();

        result.borrow_mut().weak_self = Rc::downgrade(&result);
        {
            let this = Rc::downgrade(&result);
            result.borrow().placeable_impl.borrow_mut().set_vtable(this);
        }

        result
    }

    pub(crate) fn set_measured_by_parent(&mut self, measured_by_parent: UsageByParent) {
        self.measured_by_parent = measured_by_parent;
    }

    pub(crate) fn attach(
        &mut self,
        identify: u32,
        node_chain: &Rc<RefCell<NodeChain>>,
        layout_state: &Rc<RefCell<LayoutState>>,
    ) {
        self.identify = identify;
        self.nodes = Some(node_chain.clone());
        self.layout_state = Some(layout_state.clone());
    }

    pub(crate) fn mark_measure_pending(&mut self) {
        self.measure_pending = true;
    }

    pub(crate) fn mark_layout_pending(&mut self) {
        self.layout_pending = true;
    }

    fn get_outer_coordinator(&self) -> Rc<RefCell<dyn NodeCoordinator>> {
        self.get_node_chain().borrow_mut().outer_coordinator.clone()
    }

    fn set_layout_state(&mut self, layout_state: LayoutState) {
        *self.layout_state.as_ref().unwrap().borrow_mut() = layout_state;
    }

    fn get_layout_state(&self) -> LayoutState {
        *self.layout_state.as_ref().unwrap().borrow()
    }

    pub(crate) fn perform_measure(&mut self, constraint: &Constraints) -> IntSize {
        if self.get_layout_state() != LayoutState::Idle {
            panic!("layout state is not idle before measure starts")
        }
        self.set_layout_state(LayoutState::Measuring);
        self.measure_pending = false;

        let measure_result = self.get_outer_coordinator()
            .borrow_mut()
            .measure(constraint);

        if self.get_layout_state() == LayoutState::Measuring {
            self.mark_layout_pending();
            self.set_layout_state(LayoutState::Idle);
        }

        measure_result.0
    }

    pub(crate) fn update_parent_data(&self) -> bool {
        true
    }

    fn track_measuremenet_by_parent(&mut self) {
        let parent = self.nodes.clone().unwrap().borrow().get_parent();
        if let Some(parent) = parent.unwrap_or(Weak::default()).upgrade() {
            let layout_state = parent.borrow().get_layout_state();
            self.measured_by_parent = match layout_state {
                LayoutState::Measuring => UsageByParent::InMeasureBlock,
                LayoutState::LayingOut => UsageByParent::InLayoutBlock,
                _ => {
                    panic!("Measurable could be only measured from the parent's measure or layout block. Parents state is {:?}", layout_state);
                }
            }
        }
    }

    fn get_node_chain(&self) -> Rc<RefCell<NodeChain>> {
        self.nodes.clone().unwrap()
    }

    fn get_parent(&self) -> Option<Weak<RefCell<LayoutNode>>> {
        self.get_node_chain().borrow().get_parent()
    }

    fn get_inner_coordinator(&self) -> Rc<RefCell<InnerNodeCoordinator>> {
        self.get_node_chain().borrow().inner_coordinator.clone()
    }

    pub(crate) fn mark_node_and_subtree_as_placed(&mut self) {
        self.is_placed = true
    }

    fn on_node_placed(&mut self) {
        // let parent = self.get_parent().unwrap().upgrade().unwrap();
        if !self.is_placed {
            self.mark_node_and_subtree_as_placed();
        }

        let mut new_z_index = self.get_inner_coordinator().borrow().get_z_index();

        self.get_node_chain()
            .borrow()
            .for_each_coordinator(|child| {
                new_z_index += child.get_z_index();
            });

        if new_z_index != self.z_index {
            self.z_index = new_z_index;
            // todo invalidate parent z order
        }

        self.layout_children()
    }

    fn on_before_layout_children(&mut self) {}

    fn layout_children(&mut self) {
        self.laying_out_children = true;
        if self.layout_pending {
            self.on_before_layout_children();
        }
        self.layout_pending = false;
        let layout_state = self.layout_state.as_ref().unwrap().clone();
        let old_layout_state = *layout_state.borrow_mut();
        *layout_state.borrow_mut() = LayoutState::LayingOut;

        {
            let inner_coordinator = self.get_inner_coordinator();
            let mut inner_coordinator_mut = inner_coordinator.borrow_mut();
            inner_coordinator_mut.get_measured_result().unwrap().place_children(inner_coordinator_mut.deref());
        }

        *layout_state.borrow_mut() = old_layout_state;
        self.laying_out_children = false;
    }

    fn place_outer_coordinator(&mut self, position: IntOffset, z_index: f32) {
        self.set_layout_state(LayoutState::LayingOut);
        self.last_position = position;
        self.last_z_index = z_index;
        self.place_once = true;

        if !self.layout_pending && self.is_placed {
            self.on_node_placed();
        } else {
            // todo place outer coordinator

            let outer_coordinator = self.get_outer_coordinator();
            outer_coordinator.borrow_mut().place_at(position, z_index);
            self.on_node_placed();
        }
        self.set_layout_state(LayoutState::Idle);
    }
}

impl IntrinsicMeasurable for MeasurePassDelegate {
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

impl Measurable for MeasurePassDelegate {
    fn measure(&mut self, constraint: &Constraints) -> (IntSize, Rc<RefCell<dyn Placeable>>) {
        self.track_measuremenet_by_parent();
        <Self as Remeasurable>::remeasure(self, constraint);
        let placeable = self.as_placeable();

        let measure_size = placeable.borrow().get_measured_size();
        (measure_size, placeable)
    }

    fn as_placeable(&mut self) -> Rc<RefCell<dyn Placeable>> {
        self.placeable_impl.clone()
    }

    fn as_measurable_mut(&mut self) -> &mut dyn Measurable {
        self
    }
}

impl SelfReference for MeasurePassDelegate {
    fn get_self(&self) -> Weak<RefCell<Self>> {
        self.weak_self.clone()
    }
}