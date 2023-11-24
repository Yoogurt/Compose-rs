use std::cell::RefMut;
use crate::foundation::geometry::IntSize;

use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::placeable::Placeable;

pub trait PlacementScope {
    fn parent_size(&self) -> IntSize;
    fn parent_width(&self) -> usize;
    fn parent_height(&self) -> usize;

    fn parent_layout_direction(&self) -> LayoutDirection;

    fn place(&self, placeable: RefMut<dyn Placeable>, x: i32, y: i32);
    fn place_with_z(&self, placeable: RefMut<dyn Placeable>, x: i32, y: i32, z_index: f32);
    fn place_relative(&self, placeable: RefMut<dyn Placeable>, x: i32, y: i32);
    fn place_relative_with_z(&self, placeable: RefMut<dyn Placeable>, x: i32, y: i32, z_index: f32);
}