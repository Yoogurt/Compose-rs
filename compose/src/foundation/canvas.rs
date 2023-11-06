use skia_safe::Rect;
use crate::foundation::ui::graphics::color::Color;

pub struct CanvasSaveGuard<'a> {
    pub(crate) canvas: &'a mut dyn Canvas,
}

impl Drop for CanvasSaveGuard<'_> {
    fn drop(&mut self) {
        self.canvas.restore();
    }
}

pub trait Canvas {
    fn save(&mut self) -> CanvasSaveGuard<'_>;
    fn restore(&mut self);
    fn save_count(&self) -> usize;

    fn translate(&mut self, x: f32, y:f32);

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
