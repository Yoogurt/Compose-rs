use std::time::Duration;
use crate as compose;
use minifb::{Key, KeyRepeat, Scale, ScaleMode, Window, WindowOptions};
use skia_safe::{AlphaType, ColorSpace, ColorType, ImageInfo, surfaces,
};
use crate::foundation::bridge::platform_compose_view::MacOSComposeView;
use crate::foundation::composer::Composer;
use crate::foundation::drawing::canvas_impl::new_canvas;
use crate::foundation::geometry::IntSize;

pub struct DesktopWindowOption {
    on_close_request: Option<Box<dyn Fn()>>,
    visible: bool,
    title: String,
    size: IntSize,
}

impl Default for DesktopWindowOption {
    fn default() -> Self {
        Self {
            on_close_request: None,
            visible: true,
            title: "Untitled".to_string(),
            size: IntSize::new(800, 500),
        }
    }
}

pub fn DesktopWindow(option: DesktopWindowOption,
                     content: impl Fn(),
                     diff: impl Fn()) {
    let window_width = option.size.width;
    let window_height = option.size.height;

    let mut windows = Window::new(
        &option.title,
        window_width,
        window_height,
        WindowOptions {
            scale: Scale::X1,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..Default::default()
        },
    ).unwrap();

    let mut buffer = vec![0; window_width * window_height];
    const BYTE_PER_PIXEL: usize = 4;

    let image_info = ImageInfo::new(
        (window_width as i32, window_height as i32),
        ColorType::BGRA8888,
        AlphaType::Opaque,
        Some(ColorSpace::new_srgb()),
    );

    let mut surface = unsafe {
        surfaces::wrap_pixels(
            &image_info,
            std::slice::from_raw_parts_mut(buffer.as_ptr() as *mut u8, 800 * 500 * BYTE_PER_PIXEL),
            window_width * BYTE_PER_PIXEL,
            None,
        )
    }
        .unwrap();

    let mut canvas = new_canvas(surface.canvas());
    let mut compose_view_rc = MacOSComposeView::new();
    let mut compose_view = compose_view_rc.borrow_mut();

    compose_view.set_content(content);
    drop(compose_view);

    Composer::apply_changes();
    Composer::apply_deferred_changes();

    Composer::debug_print();

    let mut compose_view = compose_view_rc.borrow_mut();
    compose_view.no_insert_set_content(diff);

    drop(compose_view);
    Composer::apply_changes();
    Composer::apply_deferred_changes();

    let mut compose_view = compose_view_rc.borrow_mut();

    while windows.is_open() && !windows.is_key_pressed(Key::Escape, KeyRepeat::No) {
        compose_view.dispatch_measure(window_width, window_height);
        compose_view.dispatch_layout();
        compose_view.dispatch_draw(&mut canvas);
        windows.update_with_buffer(buffer.as_slice(), window_width, window_height).unwrap();
        std::thread::sleep(Duration::from_millis(100));
    }
}