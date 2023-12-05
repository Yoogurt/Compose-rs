use skia_safe::canvas::SaveLayerRec;
use skia_safe::{Point, Rect, scalar};
use crate::foundation::drawing::scalar::Scalar;

use crate::foundation::ui::graphics::color::Color;

pub struct CanvasSaveGuard<'a> {
    pub(crate) canvas: &'a mut dyn Canvas,
}

impl Drop for CanvasSaveGuard<'_> {
    fn drop(&mut self) {
        self.canvas.restore();
    }
}

pub struct CanvasSaveLayerGuard<'a> {
    pub(crate) save_layer_rec: SaveLayerRec<'a>,
    pub(crate) canvas: &'a mut dyn Canvas,
}

impl Drop for CanvasSaveLayerGuard<'_> {
    fn drop(&mut self) {
        self.canvas.restore();
    }
}

pub trait Canvas {
    fn save(&mut self) -> CanvasSaveGuard<'_>;
    fn restore(&mut self);

    fn save_layer(&mut self) -> CanvasSaveLayerGuard<'_>;

    fn save_count(&self) -> usize;

    fn translate(&mut self, x: f32, y: f32);

    fn draw_circle(&mut self, point: Point, scalar: scalar, color: Color);
    fn draw_rect(&mut self, color: Color, rect: Rect);
}

pub trait CanvasExtension: Canvas {
    fn with_save<R>(&mut self, action: impl FnOnce(&mut Self) -> R) -> R {
        self.save();
        let result = action(self);
        self.restore();
        result
    }
}
