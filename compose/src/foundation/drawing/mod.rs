mod macos_canvas_impl;

pub mod canvas_impl {
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    use crate::foundation::drawing::macos_canvas_impl::MacOSCanvas;

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    pub fn new_canvas(skia_canvas: &mut skia_safe::Canvas) -> MacOSCanvas {
        MacOSCanvas::new(skia_canvas)
    }
}

pub mod scalar;
