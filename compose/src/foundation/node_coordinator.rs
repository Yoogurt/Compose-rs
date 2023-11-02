use std::{rc::Rc, rc::Weak, cell::RefCell};
use core::fmt::Debug;
use auto_delegate::Delegate;
use crate::foundation::constraint::Constraint;
use super::{placeable::{Placeable}, measurable::Measurable, layout_node::LayoutNode, measure_result::MeasureResult, look_ahead_capable_placeable::LookaheadCapablePlaceable, look_ahead_capable_placeable_impl::LookaheadCapablePlaceableImpl};
use core::any::Any;
use auto_delegate::delegate;

#[delegate]
pub trait NodeCoordinatorTrait {
    fn set_wrapped(&mut self, wrapped: Option<Rc<RefCell<dyn NodeCoordinator>>>);
    fn get_wrapped(&self) -> Option<Rc<RefCell<dyn NodeCoordinator>>>;
    fn set_wrapped_by(&mut self, wrapped_by: Option<Weak<RefCell<dyn NodeCoordinator>>>);
    fn get_wrapped_by(&self) -> Option<Rc<RefCell<dyn NodeCoordinator>>>;

    fn get_z_index(&self) -> f32;
    fn set_z_index(&mut self, z_index: f32);
}

pub trait NodeCoordinator: NodeCoordinatorTrait + Placeable + Debug + Measurable {
    fn on_initialize(&self) {}
    fn on_place(&self) {}

    fn perform_measure<'a, F>(&'a mut self, constraint: &Constraint, block: F) -> &'a mut dyn Placeable where F: FnOnce(&'a mut Self) -> &'a mut dyn Placeable, Self: Sized {
        self.set_measurement_constraint(constraint);
        block(self)
    }

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug, Delegate)]
pub(crate) struct NodeCoordinatorImpl {
    #[to(Placeable, Measured, PlaceablePlaceAt, MeasureScope)]
    pub(crate) look_ahead_capable_placeable_impl: LookaheadCapablePlaceableImpl,
    pub(crate) measure_result: MeasureResult,
    pub(crate) wrapped: Option<Rc<RefCell<dyn NodeCoordinator>>>,
    pub(crate) wrapped_by: Option<Weak<RefCell<dyn NodeCoordinator>>>,
    pub(crate) layout_node: Weak<RefCell<LayoutNode>>,
    pub(crate) z_index: f32,

    pub(crate) parent_data: Option<Box<dyn Any>>
}