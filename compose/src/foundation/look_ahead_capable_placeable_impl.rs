use std::any::Any;
use std::cell::RefCell;
use std::mem::MaybeUninit;
use std::rc::{Rc, Weak};
use crate::foundation::geometry::IntSize;
use crate::foundation::look_ahead_capable_placeable::NodeCoordinatorTrait;
use crate::foundation::utils::weak_upgrade::WeakUpdater;
use super::constraint::Constraint;
use super::layout_node::LayoutNode;
use super::layout_result::{Placeable, PlaceableImpl};
use super::look_ahead_capable_placeable::{NodeCoordinatorImpl, NodeCoordinator};
use super::measurable::Measurable;
use super::measure_result::MeasureResult;

impl Measurable for NodeCoordinatorImpl {
    fn measure(&mut self, _constraint: &Constraint) -> &mut dyn Placeable {
        unimplemented!("layout node wrapper should implement measure")
    }
}

impl NodeCoordinatorImpl {
    pub(crate) fn attach(&mut self, layout_node: Weak<RefCell<LayoutNode>>) {
        self.layout_node = Weak::new();
    }

    pub(crate)  fn layout_node(&self) -> Weak<RefCell<LayoutNode>> {
        self.layout_node.clone()
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
            placeable_impl: PlaceableImpl::new(),
            wrapped: None,
            wrapped_by: None,
            layout_node: Weak::new(),
            measure_result: MeasureResult::default(),
            parent_data: None,
        }
    }

    fn on_measure_result_changed(&mut self, size: IntSize) {
        self.set_measured_size(size);
    }

    fn set_measure_result(&mut self, measure_result: MeasureResult) {
        if self.measure_result != measure_result {
            let measure_size: (usize, usize) = measure_result.into();
            self.on_measure_result_changed(measure_size.into());
        }
    }

    pub(crate) fn on_layout_modifier_node_changed(&self) {

    }
}