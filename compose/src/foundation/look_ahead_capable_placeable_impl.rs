use std::cell::RefCell;
use std::mem::MaybeUninit;
use std::rc::Weak;
use std::ops::{Deref, DerefMut};
use crate::foundation::geometry::{IntOffset, IntSize};

use super::constraint::Constraint;
use super::layout_node::LayoutNode;
use super::layout_result::{Placeable, PlaceAction, PlaceableImpl};
use super::look_ahead_capable_placeable::{LayoutNodeWrapperImpl, LayoutNodeWrapper};
use super::measurable::Measurable;
use super::measure_result::MeasureResult;
use super::measured::Measured;

impl DerefMut for LayoutNodeWrapperImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.placeable_impl
    }
}

impl Deref for LayoutNodeWrapperImpl {
    type Target = dyn Placeable;

    fn deref(&self) -> &Self::Target {
        &self.placeable_impl
    }
}

impl Measurable for LayoutNodeWrapperImpl {
    fn measure(&mut self, _constraint: &Constraint) -> &mut dyn Placeable {
        unimplemented!("layout node wrapper should implement measure")
    }
}

impl LayoutNodeWrapper for LayoutNodeWrapperImpl {
    fn attach(&mut self, layout_node: Weak<RefCell<LayoutNode>>) {
        self.layout_node = MaybeUninit::new(layout_node);
    }

    fn layout_node(&self) -> Weak<RefCell<LayoutNode>> {
        unsafe { self.layout_node.assume_init_read() }
    }
}

impl Measured for LayoutNodeWrapperImpl {
    fn get_measured_width(&self) -> usize {
        self.get_measured_width()
    }

    fn get_measured_height(&self) -> usize {
        self.get_measured_height()
    }
}

impl Placeable for LayoutNodeWrapperImpl {
    fn get_width(&self) -> usize {
        self.placeable_impl.get_width()
    }

    fn get_height(&self) -> usize {
        self.placeable_impl.get_height()
    }

    fn get_measured_size(&self) -> IntSize {
        self.placeable_impl.get_measured_size()
    }

    fn set_measured_size(&mut self, size: IntSize) {
        self.placeable_impl.set_measured_size(size)
    }

    fn place_at(&mut self, position: IntOffset, z_index: f32, place_action: PlaceAction) {
        self.placeable_impl.place_at(position, z_index, place_action)
    }

    fn get_measurement_constraint(&self) -> &Constraint {
        self.placeable_impl.get_measurement_constraint()
    }

    fn set_measurement_constraint(&mut self, constraint: &Constraint) {
        self.placeable_impl.set_measurement_constraint(constraint)
    }

    fn perfroming_measure(&mut self, constraint: &Constraint, block: & mut dyn FnMut() -> MeasureResult) -> &dyn Placeable {
        self.placeable_impl.perfroming_measure(constraint,block)
    }
}

impl LayoutNodeWrapperImpl {
    pub(crate) fn new() -> Self {
        LayoutNodeWrapperImpl {
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