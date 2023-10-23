pub struct CanvasSaveGuard<'a> {
    pub(crate) canvas: &'a mut dyn Canvas
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
}

pub trait CanvasExtension: Canvas {
    fn with_save<R>(&mut self, action: impl FnOnce(&mut Self) -> R) -> R {
        self.save();
        let result = action(self);
        self.restore();
        result
    }
}