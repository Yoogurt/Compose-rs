use super::{measurable::Measurable, placeable::Placeable};
use crate::foundation::constraint::Constraints;
use auto_delegate::delegate;
use core::any::Any;
use core::fmt::Debug;
use std::{cell::RefCell, rc::Rc, rc::Weak};
use std::ops::Deref;
use crate::foundation::canvas::Canvas;
use crate::foundation::look_ahead_capable_placeable::LookaheadCapablePlaceable;
use crate::foundation::measure_result::MeasureResult;
use crate::foundation::modifier::ModifierNode;
use crate::foundation::oop::AnyConverter;
use crate::foundation::measure_result::MeasureResultProvider;

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
    fn get_tail(&self) -> Rc<RefCell<dyn ModifierNode>>;
}

pub trait PerformDrawTrait: NodeCoordinatorTrait {
    fn perform_draw(&self, canvas: &mut dyn Canvas) {
        if let Some(wrapped) = self.get_wrapped().as_ref() {
            wrapped.borrow().draw(canvas);
        }
    }
}

#[delegate]
pub trait DrawableNodeCoordinator {
    fn draw(&self, canvas: &mut dyn Canvas);
}

#[delegate]
pub trait NodeCoordinator: PerformDrawTrait
+ NodeCoordinatorTrait
+ LookaheadCapablePlaceable
+ TailModifierNodeProvider
+ AnyConverter
+ Placeable
+ Debug
+ MeasureResultProvider
+ DrawableNodeCoordinator
+ Measurable {
    fn on_initialize(&self) {}
    fn on_placed(&self) {}
    fn on_measured(&mut self) {}
    fn as_node_coordinator(&self) -> &dyn NodeCoordinator;
}

pub(crate) trait PerformMeasureHelper {
    fn perform_measure<F, R>(
        &mut self,
        constraint: &Constraints,
        block: F,
    ) -> R where
        F: FnOnce(&mut Self) -> R,
        Self: Sized,;
}

impl<T> PerformMeasureHelper for T where T: NodeCoordinator {
    fn perform_measure<F, R>(
        &mut self,
        constraint: &Constraints,
        block: F,
    ) -> R
        where
            F: FnOnce(&mut Self) -> R
    {
        self.set_measurement_constraint(constraint);
        block(self)
    }
}