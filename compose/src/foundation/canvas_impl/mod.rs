mod macos_canvas_impl;
pub mod canvas_impl {
    use crate::foundation::canvas_impl::macos_canvas_impl::MacOSCanvas;

    #[cfg(target_os = "macos")]
    pub fn new_canvas<'a>(skia_canvas: &'a skia_safe::Canvas) -> MacOSCanvas<'a>{
        MacOSCanvas::new(skia_canvas)
    }
}
