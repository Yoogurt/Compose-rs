use std::cell::RefCell;
use std::rc::Rc;
use crate::foundation::geometry::{IntOffset, IntSize};
use crate::foundation::look_ahead_capable_placeable::LookaheadCapablePlaceable;
use crate::foundation::measure_scope::MeasureScopeImpl;
use crate::foundation::placeable_impl::PlaceableImpl;
use crate::foundation::placeable_place_at::PlaceablePlaceAt;
use auto_delegate::Delegate;
use crate::foundation::constraint::Constraints;
use crate::foundation::measured::Measured;
use crate::foundation::placeable::Placeable;

#[derive(Default, Debug, Delegate)]
pub(crate) struct LookaheadCapablePlaceableImpl {
    placeable_impl: Rc<RefCell<PlaceableImpl>>,
    #[to(MeasureScope)]
    measure_scope_impl: MeasureScopeImpl,
    position: IntOffset,
}

impl Measured for LookaheadCapablePlaceableImpl {
    fn get_measured_width(&self) -> usize {
        self.placeable_impl.borrow().get_measured_width()
    }

    fn get_measured_height(&self) -> usize {
        self.placeable_impl.borrow().get_measured_height()
    }
}

impl Placeable for LookaheadCapablePlaceableImpl {
    fn get_size(&self) -> IntSize {
        self.placeable_impl.borrow().get_size()
    }

    fn set_measured_size(&mut self, size: IntSize) {
        self.placeable_impl.borrow_mut().set_measured_size(size)
    }

    fn get_measured_size(&self) -> IntSize {
        self.placeable_impl.borrow().get_measured_size()
    }

    fn set_measurement_constraint(&mut self, constraint: &Constraints) {
        self.placeable_impl.borrow_mut().set_measurement_constraint(constraint)
    }
    fn get_measurement_constraint(&self) -> Constraints {
        self.placeable_impl.borrow().get_measurement_constraint()
    }
}

impl PlaceablePlaceAt for LookaheadCapablePlaceableImpl {
    fn place_at(&mut self, _position: IntOffset, _z_index: f32) {
        unimplemented!("unimplemented place_at for LookaheadCapablePlaceableImpl")
    }
}

impl LookaheadCapablePlaceable for LookaheadCapablePlaceableImpl {
    fn set_position(&mut self, position: IntOffset) {
        self.position = position;
    }

    fn get_position(&self) -> IntOffset {
        self.position
    }
}

impl LookaheadCapablePlaceableImpl {
    pub(crate) fn as_placeable(&self) -> Rc<RefCell<dyn Placeable>> {
        self.placeable_impl.clone()
    }
}