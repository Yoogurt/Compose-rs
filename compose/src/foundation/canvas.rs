use skia_safe::canvas::SaveLayerRec;
use skia_safe::{Point, Rect, scalar};
use crate::foundation::drawing::scalar::Scalar;

use crate::foundation::ui::graphics::color::Color;

pub trait Canvas {
    fn save(&mut self);
    fn restore(&mut self);

    fn save_layer(&mut self) -> SaveLayerRec;
    fn save_layer_alpha(&mut self, rect: Option<Rect>, alpha: f32);

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

    fn with_save_layer<R>(&mut self, action: impl FnOnce(&mut Self) -> R) -> R {
        let _ = self.save_layer();
        let result = action(self);
        self.restore();
        result
    }

    fn with_save_layer_alpha<R>(&mut self, rect: Option<Rect>, alpha: f32, action: impl FnOnce(&mut Self) -> R) -> R {
        let _ = self.save_layer_alpha(rect, alpha);
        let result = action(self);
        self.restore();
        result
    }
}
