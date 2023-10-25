use crate::foundation::canvas::{Canvas, CanvasSaveGuard, CanvasExtension};

pub struct MacOSCanvas<'a> {
    inner: &'a mut skia_safe::Canvas,
}

impl<'a> MacOSCanvas<'a> {
    pub fn new(skia_canvas: &'a mut skia_safe::Canvas) -> MacOSCanvas {
        MacOSCanvas {
            inner: skia_canvas
        }
    }
}



impl Canvas for MacOSCanvas<'_> {
    fn save(&mut self) -> CanvasSaveGuard<'_> {
        self.inner.save();
        CanvasSaveGuard {
            canvas: self
        }
    }

    fn restore(&mut self) {
        self.inner.restore();
    }

    fn save_count(&self) -> usize {
        self.inner.save_count()
    }
}

impl CanvasExtension for MacOSCanvas<'_> {}