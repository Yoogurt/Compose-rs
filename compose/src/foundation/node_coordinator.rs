use std::{rc::Rc, rc::Weak, cell::RefCell};
use core::fmt::Debug;
use crate::foundation::constraint::Constraints;
use super::{placeable::{Placeable}, measurable::Measurable};
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
    fn on_measured(&mut self) {}

    fn perform_measure<'a, F>(&'a mut self, constraint: &Constraints, block: F) -> &'a mut dyn Placeable where F: FnOnce(&'a mut Self) -> &'a mut dyn Placeable, Self: Sized {
        self.set_measurement_constraint(constraint);
        block(self)
    }

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
