use super::constraint::Constraints;
use super::layout_node::LayoutNode;
use super::measurable::Measurable;
use super::node_coordinator::NodeCoordinator;
use super::placeable::Placeable;
use crate::foundation::geometry::{IntOffset, IntSize};
use crate::foundation::intrinsic_measurable::IntrinsicMeasurable;
use crate::foundation::look_ahead_capable_placeable::LookaheadCapablePlaceable;
use crate::foundation::look_ahead_capable_placeable_impl::LookaheadCapablePlaceableImpl;
use crate::foundation::node_coordinator::NodeCoordinatorTrait;
use crate::foundation::placeable_place_at::PlaceablePlaceAt;
use crate::foundation::utils::weak_upgrade::WeakUpdater;
use auto_delegate::Delegate;
use std::any::Any;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

#[derive(Debug, Delegate)]
pub(crate) struct NodeCoordinatorImpl {
    #[to(Placeable, Measured, MeasureScope)]
    pub(crate) look_ahead_capable_placeable_impl: LookaheadCapablePlaceableImpl,
    // pub(crate) measure_result: MeasureResult,
    pub(crate) wrapped: Option<Rc<RefCell<dyn NodeCoordinator>>>,
    pub(crate) wrapped_by: Option<Weak<RefCell<dyn NodeCoordinator>>>,
    pub(crate) layout_node: Weak<RefCell<LayoutNode>>,
    pub(crate) z_index: f32,

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

impl Deref for NodeCoordinatorImpl {
    type Target = dyn LookaheadCapablePlaceable;

    fn deref(&self) -> &Self::Target {
        &self.look_ahead_capable_placeable_impl
    }
}

impl DerefMut for NodeCoordinatorImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.look_ahead_capable_placeable_impl
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

impl NodeCoordinator for NodeCoordinatorImpl {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl NodeCoordinatorImpl {
    pub(crate) fn new() -> Self {
        NodeCoordinatorImpl {
            look_ahead_capable_placeable_impl: LookaheadCapablePlaceableImpl::default(),
            wrapped: None,
            wrapped_by: None,
            layout_node: Weak::new(),
            // measure_result: MeasureResult::default(),
            parent_data: None,
            z_index: 0.0,
        }
    }

    // fn on_measure_result_changed(&mut self, size: IntSize) {
    //     self.set_measured_size(size);
    // }

    // fn set_measure_result(&mut self, measure_result: MeasureResult) {
    //     if self.measure_result != measure_result {
    //         let measure_size: (usize, usize) = measure_result.into();
    //         self.on_measure_result_changed(measure_size.into());
    //     }
    // }

    pub(crate) fn on_layout_modifier_node_changed(&self) {}

    fn place_self(&mut self, position: IntOffset, z_index: f32) {
        if self.get_position() != position {
            self.set_position(position);
        }

        self.z_index = z_index;
    }
}

impl PlaceablePlaceAt for NodeCoordinatorImpl {
    fn place_at(&mut self, position: IntOffset, z_index: f32) {
        self.place_self(position, z_index)
    }
}
