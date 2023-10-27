use std::cell::RefCell;
use std::mem::MaybeUninit;
use std::rc::Weak;
use crate::foundation::geometry::IntSize;
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

impl NodeCoordinator for NodeCoordinatorImpl {
    fn attach(&mut self, layout_node: Weak<RefCell<LayoutNode>>) {
        self.layout_node = MaybeUninit::new(layout_node);
    }

    fn layout_node(&self) -> Weak<RefCell<LayoutNode>> {
        unsafe { self.layout_node.assume_init_read() }
    }
}

impl NodeCoordinatorImpl {
    pub(crate) fn new() -> Self {
        NodeCoordinatorImpl {
            placeable_impl: PlaceableImpl::new(),
            wrapped_by: None,
            layout_node: MaybeUninit::uninit(),
            measure_result: MeasureResult::default(),
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
}