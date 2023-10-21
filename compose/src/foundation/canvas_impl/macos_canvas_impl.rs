use crate::foundation::Canvas;

pub struct MacOSCanvas<'a> {
    inner: &'a skia_safe::Canvas,
}

impl<'a> MacOSCanvas<'a> {
    pub fn new(skia_canvas: &'a skia_safe::Canvas) -> MacOSCanvas {
        MacOSCanvas {
            inner: skia_canvas
        }
    }
}

impl Canvas for MacOSCanvas<'_> {
    fn save(&self) {
        self.inner.save();
    }

    fn restore(&self) {
        self.inner.restore();
    }
}