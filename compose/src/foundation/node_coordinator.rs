use super::{measurable::Measurable, placeable::Placeable};
use crate::foundation::constraint::Constraints;
use auto_delegate::delegate;
use core::any::Any;
use core::fmt::Debug;
use std::{cell::RefCell, rc::Rc, rc::Weak};
use std::ops::Deref;
use crate::foundation::canvas::Canvas;
use crate::foundation::look_ahead_capable_placeable::LookaheadCapablePlaceable;
use crate::foundation::modifier::ModifierNode;
use crate::foundation::oop::AnyConverter;

#[delegate]
pub trait NodeCoordinatorTrait {
    fn set_wrapped(&mut self, wrapped: Option<Rc<RefCell<dyn NodeCoordinator>>>);
    fn get_wrapped(&self) -> Option<Rc<RefCell<dyn NodeCoordinator>>>;
    fn set_wrapped_by(&mut self, wrapped_by: Option<Weak<RefCell<dyn NodeCoordinator>>>);
    fn get_wrapped_by(&self) -> Option<Rc<RefCell<dyn NodeCoordinator>>>;

    fn get_z_index(&self) -> f32;
    fn set_z_index(&mut self, z_index: f32);
}

#[delegate]
pub trait TailModifierNodeProvider {
    fn set_tail(&mut self, tail: Rc<RefCell<dyn ModifierNode>>);
    fn get_tail(&self) -> Rc<RefCell<dyn ModifierNode>>;
}

pub trait PerformDrawTrait: NodeCoordinatorTrait {
    fn perform_draw(&self, canvas: &mut dyn Canvas) {
        if let Some(wrapped) = self.get_wrapped().as_ref() {
            wrapped.borrow_mut().draw(canvas);
        }
    }
}

#[delegate]
pub trait NodeCoordinator: PerformDrawTrait
+ NodeCoordinatorTrait
+ LookaheadCapablePlaceable
+ TailModifierNodeProvider
+ AnyConverter
+ Placeable
+ Debug
+ Measurable {
    fn on_initialize(&self) {}
    fn on_place(&self) {}
    fn on_measured(&mut self) {}
    fn as_node_coordinator(&self) -> &dyn NodeCoordinator;

    fn draw(&self, canvas: &mut dyn Canvas);
}

pub(crate) trait PerformMeasureHelper {
    fn perform_measure<'a, F>(
        &'a mut self,
        constraint: &Constraints,
        block: F,
    ) -> &'a mut dyn Placeable where
        F: FnOnce(&'a mut Self) -> &'a mut dyn Placeable,
        Self: Sized,;
}

impl<T> PerformMeasureHelper for T where T: NodeCoordinator {
    fn perform_measure<'a, F>(
        &'a mut self,
        constraint: &Constraints,
        block: F,
    ) -> &mut dyn Placeable
        where
            F: FnOnce(&'a mut Self) -> &'a mut dyn Placeable
    {
        self.set_measurement_constraint(constraint);
        block(self)
    }
}