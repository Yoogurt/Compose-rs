use auto_delegate::delegate;
use crate::foundation::geometry::IntOffset;

#[delegate]
pub trait PlaceablePlaceAt {
    fn place_at(&mut self, position: IntOffset, z_index: f32);
}