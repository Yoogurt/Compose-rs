use core::fmt::Debug;
use std::{cell::RefCell, rc::Rc, rc::Weak};
use std::alloc::Layout;
use std::cell::Ref;

use auto_delegate::delegate;

use crate::foundation::layout_node::LayoutNode;
use crate::foundation::canvas::Canvas;
use crate::foundation::constraint::Constraints;
use crate::foundation::geometry::Offset;
use crate::foundation::layout::layout_coordinates::LayoutCoordinates;
use crate::foundation::look_ahead_capable_placeable::LookaheadCapablePlaceable;
use crate::foundation::measure_result::MeasureResultProvider;
use crate::foundation::modifier::{ModifierNode, NodeKind};
use crate::foundation::node::OwnedLayer;
use crate::foundation::node_coordinator_impl::{NodeCoordinatorImpl, PointerInputSource};
use crate::foundation::oop::AnyConverter;
use crate::foundation::parent_data::ParentDataGenerator;
use crate::foundation::ui::hit_test_result::HitTestResult;

use super::{measurable::Measurable, placeable::Placeable};

#[delegate]
pub(crate) trait NodeCoordinatorTrait {
    fn set_wrapped(&mut self, wrapped: Option<Rc<RefCell<dyn NodeCoordinator>>>);
    fn get_wrapped(&self) -> Option<Rc<RefCell<dyn NodeCoordinator>>>;
    fn set_wrapped_by(&mut self, wrapped_by: Option<Weak<RefCell<dyn NodeCoordinator>>>);
    fn get_wrapped_by(&self) -> Option<Rc<RefCell<dyn NodeCoordinator>>>;

    fn get_z_index(&self) -> f32;
    fn set_z_index(&mut self, z_index: f32);
}

#[delegate]
pub(crate) trait TailModifierNodeProvider {
    fn get_tail(&self) -> Rc<RefCell<dyn ModifierNode>>;
}

pub(crate) trait PerformDrawTrait: NodeCoordinatorTrait {
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
pub trait AsNodeCoodinator {
    fn node_coordinator_ref(&self) -> &NodeCoordinatorImpl;
}

#[delegate]
pub trait HitTestTrait {
    fn hit_test(&self, hit_test_source: &dyn HitTestSource, pointer_position: Offset<f32>, hit_test_result: &mut HitTestResult, is_touch_event: bool, is_in_layer:bool);
}

#[delegate]
pub(crate) trait NodeCoordinator: PerformDrawTrait
+ NodeCoordinatorTrait
+ LookaheadCapablePlaceable
+ TailModifierNodeProvider
+ AnyConverter
+ Placeable
+ Debug
+ MeasureResultProvider
+ DrawableNodeCoordinator
+ ParentDataGenerator
+ LayoutCoordinates
+ AsNodeCoodinator
+ HitTestTrait
+ Measurable {
    fn on_initialize(&self) {}
    fn on_placed(&self) {}
    fn on_measured(&mut self) {}

    fn get_layer(&self) -> Option<&Box<dyn OwnedLayer>>;

    fn from_parent_position(&self, position: Offset<f32>) -> Offset<f32> {
        let relative_to_position = position - self.get_position().as_f32_offset();

        self.get_layer().map_or(relative_to_position, |layer| {
            layer.map_offset(relative_to_position, false)
        })
    }
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

pub(crate) trait HitTestSource {
    fn entity_type(&self) -> NodeKind;
    fn intercept_out_of_bounds_child_events(&self, node: Rc<RefCell<dyn ModifierNode>>) -> bool;
    fn should_hit_Test_children(&self, parnet_layout_node: Rc<RefCell<LayoutNode>>) -> bool;

    fn child_hit_test(&self, layout_node: Rc<RefCell<LayoutNode>>, pointer_position: Offset<f32>, hit_test_result: &mut HitTestResult, is_touch_event: bool, is_in_layer: bool);
}