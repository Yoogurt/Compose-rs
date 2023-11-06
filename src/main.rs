#![allow(warnings)]

use compose::foundation::bridge::platform_compose_view::MacOSComposeView;
use compose::foundation::composer::Composer;
use compose::foundation::drawing::canvas_impl::new_canvas;
use compose::foundation::geometry::IntoDp;
use compose::foundation::layout::size_modifier::SizeModifier;
use compose::foundation::modifier::Modifier;
use compose_macro::Composable;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use skia_safe::{surfaces, AlphaType, Color, ColorSpace, ColorType, ImageInfo, Rect, Surface,
};
use std::default::Default as STDefault;
use std::hash::Hash;
use std::time::Duration;
use compose::widgets::r#box::BoxLayout;

#[Composable]
fn test_box_composable() {
    BoxLayout(Modifier.width(10.dp()), |scope| {
        BoxLayout(scope.match_parent_size(Modifier), |_| {});
    })
}

fn run_skia(content: fn()) {
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

    let mut buffer = vec![0; 800 * 500];
    const BYTE_PER_PIXEL: usize = 4;

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
    }
        .unwrap();

    let mut painter = skia_safe::Paint::default();
    painter.set_color(<u32 as Into<Color>>::into(0xff0000ffu32));

    let mut canvas = new_canvas(surface.canvas());
    let mut compose_view = MacOSComposeView::new();
    compose_view.set_content(content);

    // while window.is_open() && !window.is_key_down(Key::Escape) {
    //     std::thread::sleep(Duration::from_millis(5000));

    compose_view.dispatch_measure(800, 500);
    compose_view.dispatch_draw(&mut canvas);

    std::thread::sleep(Duration::from_millis(10000));
    // }
}

fn main() {
    run_skia(|| {
        test_box_composable();
    });

    Composer::apply_changes();
    Composer::apply_deferred_changes();
    Composer::validate_group();

    Composer::destroy();

    compose::foundation::memory::leak_token::validate_leak();
}
