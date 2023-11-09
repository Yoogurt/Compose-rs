use std::cell::RefCell;
use std::rc::Weak;
use crate::foundation::constraint::Constraints;
use crate::foundation::geometry::{CoerceIn, IntOffset, IntSize};
use crate::foundation::measured::MeasuredImpl;
use crate::foundation::placeable::Placeable;
use crate::foundation::placeable_place_at::PlaceablePlaceAt;
use auto_delegate::Delegate;

#[derive(Debug, Delegate, Default)]
pub(crate) struct PlaceableImpl {
    pub(crate) size: IntSize,
    #[to(Measured)]
    pub(crate) measured: MeasuredImpl,
    pub(crate) measured_size: IntSize,
    pub(crate) measurement_constraint: Constraints,
    pub(crate) apparent_to_real_offset: IntOffset,

    place_at_vtable: Option<Weak<RefCell<dyn PlaceablePlaceAt>>>,
}

impl PlaceableImpl {
    pub(crate) fn new() -> Self {
        PlaceableImpl {
            size: IntSize::zero(),
            measured: MeasuredImpl::new(),
            measured_size: IntSize::zero(),
            measurement_constraint: Constraints::unbounded(),
            apparent_to_real_offset: IntOffset::zero(),

            place_at_vtable: None,
        }
    }

    pub(crate) fn set_vtable(&mut self, place_at: Weak<RefCell<dyn PlaceablePlaceAt>>) {
        self.place_at_vtable = Some(place_at);
    }
}

impl PlaceableImpl {
    fn recalculate_width_and_height(&mut self) {
        *self.size.width_mut() = self
            .measured_size
            .width()
            .coerce_in(self.measurement_constraint.width_range());
        *self.size.height_mut() = self
            .measured_size
            .height()
            .coerce_in(self.measurement_constraint.height_range());

        self.apparent_to_real_offset =
            IntOffset::new((self.size.width() as i32 - self.measured_size.width() as i32) / 2,
                           (self.size.height() as i32 - self.measured_size.height() as i32) / 2)
    }
}

impl PlaceablePlaceAt for PlaceableImpl {
    fn place_at(&mut self, position: IntOffset, z_index: f32) {
        if let Some(vtable) = self.place_at_vtable.clone() {
            vtable.upgrade().unwrap().borrow_mut().place_at(position, z_index);
            return;
        }
        unimplemented!("place_at to PlaceableImpl should implement by yourself");
    }
}

impl Placeable for PlaceableImpl {
    fn get_size(&self) -> IntSize {
        self.size
    }

    fn set_measured_size(&mut self, size: IntSize) {
        self.measured_size = size;
        self.recalculate_width_and_height();
    }

    fn get_measured_size(&self) -> IntSize {
        self.measured_size
    }

    fn set_measurement_constraint(&mut self, constraint: &Constraints) {
        self.measurement_constraint = *constraint;
        self.recalculate_width_and_height()
    }

    fn get_measurement_constraint(&self) -> Constraints {
        self.measurement_constraint
    }
}
