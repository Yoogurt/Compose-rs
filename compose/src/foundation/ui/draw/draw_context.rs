use crate::foundation::geometry::{Density, Size};
use crate::foundation::canvas::Canvas;

pub struct DrawContext<'a> {
    density: Density,
    size: Size<f32>,
    canvas: &'a mut dyn Canvas,
}

impl<'a> DrawContext<'a> {
    pub fn new(size: Size<f32>, density: Density, canvas: &'a mut dyn Canvas) -> Self {
        Self {
            density,
            size,
            canvas,
        }
    }

    pub fn get_size(&self) -> Size<f32> {
        self.size
    }

    pub fn get_density(&self) -> Density {
        self.density
    }

    pub fn get_canvas(&mut self) -> &mut dyn Canvas {
        &mut *self.canvas
    }
}