#![allow(warnings)]

use std::hash::Hash;
use std::default::Default as STDefault;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use skia_safe::{
    surface, surfaces, AlphaType, Color, Color4f, ColorSpace, ColorType, ImageInfo, Rect, Surface,
};
use std::time::Duration;
use skia_safe::canvas::lattice::RectType::Default;
use compose::Box;
use compose::foundation::{Composer, Constraint, Modifier};
use compose::foundation::bridge::platform_compose_view::MacOSComposeView;
use compose_macro::Compose;
use compose::foundation::canvas_impl::canvas_impl::new_canvas;

#[Compose]
fn test() {
    Box! {
        Box! {

        }
    }
}

fn run_skia() {
    // test(1, 2);
    let mut window = Window::new(
        "Compose",
        800,
        500,
        WindowOptions {
            scale: Scale::X1,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..STDefault::default()
        },
    )
        .unwrap();

    let mut buffer = [0u32; 800 * 500];
    const BYTE_PER_PIXEL: usize = 4;

    test();

    let image_info = ImageInfo::new(
        (800, 500),
        ColorType::BGRA8888,
        AlphaType::Opaque,
        Some(ColorSpace::new_srgb()),
    );

    let mut surface = unsafe {
        surfaces::wrap_pixels(
            &image_info,
            std::slice::from_raw_parts_mut(buffer.as_ptr() as *mut u8, 800 * 500 * BYTE_PER_PIXEL),
            800 * BYTE_PER_PIXEL,
            None,
        )
    }.unwrap();

    let mut painter = skia_safe::Paint::default();
    painter.set_color(<u32 as Into<Color>>::into(0xff0000ffu32));

    let canvas = new_canvas(surface.canvas());
    let mut compose_view = MacOSComposeView::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        std::thread::sleep(Duration::from_millis(500));

        compose_view.dispatch_measure(800, 500);
        compose_view.dispatch_draw(&canvas);
    }
}

fn main() {
    run_skia()
}
