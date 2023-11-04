use crate::foundation::geometry::IntOffset;
use auto_delegate::delegate;

#[delegate]
pub trait PlaceablePlaceAt {
    fn place_at(&mut self, position: IntOffset, z_index: f32);
}
