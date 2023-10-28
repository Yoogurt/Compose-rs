use std::{rc::Weak, cell::RefCell, mem::MaybeUninit};
use core::fmt::Debug;
use auto_delegate::Delegate;
use crate::foundation::constraint::Constraint;

use super::{layout_result::{Placeable, PlaceableImpl}, measurable::Measurable, layout_node::LayoutNode, measure_result::MeasureResult};

pub(crate) trait NodeCoordinator: Placeable + Debug + Measurable {
    fn attach(&mut self, layout_node: Weak<RefCell<LayoutNode>>);
    fn layout_node(&self) -> Weak<RefCell<LayoutNode>>;
    fn on_initialize(&self) {}
    fn on_place(&self) {}

    fn perform_measure<'a, F>(&'a mut self, constraint: &Constraint, block: F) -> &'a mut dyn Placeable where F: FnOnce(&'a mut Self) -> &'a mut dyn Placeable, Self: Sized {
        self.set_measurement_constraint(constraint);
        block(self)
    }
}

#[derive(Debug, Delegate)]
pub(crate) struct NodeCoordinatorImpl {
    #[to(Placeable, Measured)]
    pub(crate) placeable_impl: PlaceableImpl,
    pub(crate) measure_result: MeasureResult,
    pub(crate) wrapped_by: Option<Box<dyn NodeCoordinator>>,
    pub(crate) layout_node: MaybeUninit<Weak<RefCell<LayoutNode>>>,
}