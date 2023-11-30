use auto_delegate::delegate;
use crate::foundation::geometry::IntSize;

#[delegate]
pub trait LayoutCoordinates {
    fn size(&self) -> IntSize;

    fn is_attached(&self) -> bool;
}