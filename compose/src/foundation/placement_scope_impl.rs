use std::cell::RefMut;

use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::measure_scope::MeasureScope;
use crate::foundation::placeable::Placeable;
use crate::foundation::placement_scope::PlacementScope;

pub(crate) struct PlacementScopeImpl<'a> {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) measure_scope: &'a dyn MeasureScope,
}

impl PlacementScope for PlacementScopeImpl<'_> {
    fn parent_width(&self) -> usize {
        self.width
    }

    fn parent_height(&self) -> usize {
        self.height
    }

    fn parent_layout_direction(&self) -> LayoutDirection {
        self.measure_scope.get_layout_direction()
    }

    fn place(&self, mut placeable: RefMut<dyn Placeable>, x: i32, y: i32) {
        self.place_with_z(placeable, x, y, 0f32)
    }

    fn place_with_z(&self, mut placeable: RefMut<dyn Placeable>, x: i32, y: i32, z_index: f32) {
        placeable.place_at((x, y).into(), z_index)
    }

    fn place_relative(&self, placeable: RefMut<dyn Placeable>, x: i32, y: i32) {
        self.place_relative_with_z(placeable, x, y, 0.0)
    }

    fn place_relative_with_z(&self, mut placeable: RefMut<dyn Placeable>, x: i32, y: i32, z_index: f32) {
        // mirror
        if self.parent_layout_direction() == LayoutDirection::Ltr || self.parent_width() == 0 {
            placeable.place_at((x, y).into(), z_index)
        } else {
            let x = self.parent_width() as i32 - placeable.get_size().width as i32 - x;

            placeable.place_at(
                (
                    x,
                    y,
                )
                    .into(),
                z_index,
            )
        }
    }
}

impl<'a> PlacementScopeImpl<'a> {
    pub(crate) fn new(width: usize, height: usize, measure_scope: &'a dyn MeasureScope) -> Self {
        PlacementScopeImpl {
            width,
            height,
            measure_scope,
        }
    }
}
