use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use crate::foundation::geometry::IntSize;

use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::measure_scope::MeasureScope;
use crate::foundation::placeable::Placeable;
use crate::foundation::placement_scope::PlacementScope;
use crate::foundation::ui::graphics::graphics_layer_modifier::GraphicsLayerScope;

pub(crate) struct PlacementScopeImpl<'a> {
    pub(crate) size: IntSize,
    pub(crate) measure_scope: &'a dyn MeasureScope,
}

impl PlacementScope for PlacementScopeImpl<'_> {
    fn parent_size(&self) -> IntSize {
        self.size
    }

    fn parent_width(&self) -> usize {
        self.size.width
    }

    fn parent_height(&self) -> usize {
        self.size.height
    }

    fn parent_layout_direction(&self) -> LayoutDirection {
        self.measure_scope.get_layout_direction()
    }

    fn place(&self, mut placeable: &Rc<RefCell<dyn Placeable>>, x: i32, y: i32) {
        self.place_with_z(placeable, x, y, 0f32)
    }

    fn place_with_z(&self, mut placeable: &Rc<RefCell<dyn Placeable>>, x: i32, y: i32, z_index: f32) {
        placeable.borrow_mut().place_at((x, y).into(), z_index, None)
    }

    fn place_relative(&self, placeable: &Rc<RefCell<dyn Placeable>>, x: i32, y: i32) {
        self.place_relative_with_z(placeable, x, y, 0.0)
    }

    fn place_relative_with_z(&self, mut placeable: &Rc<RefCell<dyn Placeable>>, x: i32, y: i32, z_index: f32) {
        let mut placeable = placeable.borrow_mut();
        // mirror
        if self.parent_layout_direction() == LayoutDirection::Ltr || self.parent_width() == 0 {
            placeable.place_at((x, y).into(), z_index, None)
        } else {
            let x = self.parent_width() as i32 - placeable.get_size().width as i32 - x;

            placeable.place_at(
                (
                    x,
                    y,
                )
                    .into(),
                z_index,
                None
            )
        }
    }

    fn place_with_layer(&self, placeable: &Rc<RefCell<dyn Placeable>>, x: i32, y: i32, z_index: f32, layer_block: Rc<dyn Fn(&mut GraphicsLayerScope) + 'static>) {
        placeable.borrow_mut().place_at((x, y).into(), z_index, Some(layer_block))
    }
}

impl<'a> PlacementScopeImpl<'a> {
    pub(crate) fn new(width: usize, height: usize, measure_scope: &'a dyn MeasureScope) -> Self {
        PlacementScopeImpl {
            size: IntSize::new(width, height),
            measure_scope,
        }
    }
}
