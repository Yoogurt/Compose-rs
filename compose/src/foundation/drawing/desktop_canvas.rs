use skia_safe::{Color4f, Paint, Point, Rect, scalar, Vector};
use skia_safe::canvas::SaveLayerRec;

use crate::foundation::canvas::{Canvas, CanvasExtension};
use crate::foundation::drawing::scalar::Scalar;
use crate::foundation::ui::graphics::color;
use crate::foundation::ui::graphics::color::Color;

pub struct DesktopCanvas<'a> {
    paint: Paint,
    inner: &'a mut skia_safe::Canvas,
}

impl<'a> DesktopCanvas<'a> {
    pub fn new(skia_canvas: &'a mut skia_safe::Canvas) -> DesktopCanvas {
        DesktopCanvas {
            paint: Paint::new(Color4f::from(Color::WHITE), None),
            inner: skia_canvas,
        }
    }
}

impl Canvas for DesktopCanvas<'_> {
    fn save(&mut self) {
        self.inner.save();
    }

    fn restore(&mut self) {
        self.inner.restore();
    }

    fn save_layer(&mut self) -> SaveLayerRec {
        let save_layer_rec = SaveLayerRec::default();
        self.inner.save_layer(&save_layer_rec);
        return save_layer_rec;
    }

    fn save_count(&self) -> usize {
        self.inner.save_count()
    }

    fn translate(&mut self, x: f32, y: f32) {
        self.inner.translate(Vector::new(x, y));
    }

    fn draw_circle(&mut self, point: Point, scalar: scalar, color: Color) {
        self.paint.set_color(color);
        self.inner.draw_circle(point, scalar, &self.paint);
    }

    fn draw_rect(&mut self, color: Color, rect: Rect) {
        self.paint.set_color(color);
        self.inner.draw_rect(rect, &self.paint);
    }
}

impl<T> CanvasExtension for T where T: ?Sized + Canvas {}