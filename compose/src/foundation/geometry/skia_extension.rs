use crate::foundation::geometry::Offset;

pub(crate) trait SkiaPointExtension {
    fn as_offset(&self) -> Offset<f32>;
}

impl SkiaPointExtension for skia_safe::Point {
    fn as_offset(&self) -> Offset<f32> {
        Offset::new(self.x, self.y)
    }
}