use skia_safe::{Color4f, Paint, Rect, Vector};
use crate::foundation::canvas::{Canvas, CanvasExtension, CanvasSaveGuard};
use crate::foundation::ui::graphics::color;
use crate::foundation::ui::graphics::color::Color;

pub struct MacOSCanvas<'a> {
    inner: &'a mut skia_safe::Canvas,
}

impl<'a> MacOSCanvas<'a> {
    pub fn new(skia_canvas: &'a mut skia_safe::Canvas) -> MacOSCanvas {
        MacOSCanvas { inner: skia_canvas }
    }
}

impl Canvas for MacOSCanvas<'_> {
    fn save(&mut self) -> CanvasSaveGuard<'_> {
        self.inner.save();
        CanvasSaveGuard { canvas: self }
    }

    fn restore(&mut self) {
        self.inner.restore();
    }

    fn save_count(&self) -> usize {
        self.inner.save_count()
    }

    fn translate(&mut self, x: f32, y: f32) {
        self.inner.translate(Vector::new(x, y));
    }

    fn draw_rect(&mut self, color: Color, rect: Rect) {
        let paint = Paint::new(<color::Color as Into<Color4f>>::into(color), None);
        self.inner.draw_rect(rect, &paint);
    }
}

impl CanvasExtension for MacOSCanvas<'_> {}