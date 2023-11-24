#![allow(warnings)]

use std::default::Default as STDefault;
use std::hash::Hash;
use std::time::Duration;

use compose::foundation::background::BackgroundModifier;
use compose::foundation::bridge::platform_compose_view::MacOSComposeView;
use compose::foundation::composer::{Composer, ScopeUpdateScopeHelper};
use compose::foundation::drawing::canvas_impl::new_canvas;
use compose::foundation::geometry::IntoDp;
use compose::foundation::layout::size_modifier::SizeModifier;
use compose::foundation::modifier::Modifier;
use compose::foundation::spacer::Spacer;
use compose::foundation::ui::align::Alignment;
use compose::foundation::ui::graphics::color::Color;
use compose::widgets::r#box::BoxLayout;
use compose::widgets::row::{Row, RowParams};
use compose_macro::Composable;
use minifb::{Scale, ScaleMode, Window, WindowOptions};
use skia_safe::{AlphaType, ColorSpace, ColorType, ImageInfo, surfaces,
};

fn test_widget() {
    Row(Modifier.padding_top(100.dp()).padding_start(50.dp()).width(200.dp()).height(200.dp()).background(Color::BLUE), RowParams {
        ..Default::default()
    }, |row_scope| {
        BoxLayout(Modifier.height(100.dp()).weight(row_scope, 1f32).vertical_align(row_scope, Alignment::CENTER_VERTICALLY).background(Color::YELLOW), |_| {});
        Spacer(Modifier.width(50.dp()));
    });
}

fn test_widget_move() {
    Row(Modifier.padding_top(100.dp()).padding_start(50.dp()).width(200.dp()).height(200.dp()).background(Color::BLUE), RowParams {
        ..Default::default()
    }, |row_scope| {
        BoxLayout(Modifier.height(100.dp()).weight(row_scope, 1f32).vertical_align(row_scope, Alignment::CENTER_VERTICALLY).background(Color::YELLOW), |_| {});
    });
}

fn run_skia_render_engine(content: impl Fn(), diff: impl Fn()) {
    let mut windows = Window::new(
        "Compose-rs",
        800,
        500,
        WindowOptions {
            scale: Scale::X1,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..STDefault::default()
        },
    ).unwrap();

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

    // while windows.is_open() && !windows.is_key_pressed(Key::Escape, KeyRepeat::No) {
    //     compose_view.dispatch_measure(800, 500);
    //     compose_view.dispatch_layout();
    //     compose_view.dispatch_draw(&mut canvas);
    //     windows.update_with_buffer(buffer.as_slice(), 800, 500).unwrap();
    //     std::thread::sleep(Duration::from_millis(100));
    // }
}

fn main() {
    run_skia_render_engine(|| {
        test_widget();
    }, || {
        test_widget_move();
    });

    Composer::validate_group();
    Composer::debug_print();
    Composer::destroy();
    compose::foundation::memory::leak_token::validate_leak();
}
