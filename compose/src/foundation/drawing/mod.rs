mod desktop_canvas;

pub mod canvas_impl {
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    use crate::foundation::drawing::desktop_canvas::DesktopCanvas;

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    pub fn new_canvas(skia_canvas: &mut skia_safe::Canvas) -> DesktopCanvas {
        DesktopCanvas::new(skia_canvas)
    }
}

pub mod scalar;
