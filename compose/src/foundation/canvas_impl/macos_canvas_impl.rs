use crate::foundation::Canvas;

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
    fn save(&mut self) {
        self.inner.save();
    }

    fn restore(&mut self) {
        self.inner.restore();
    }
}