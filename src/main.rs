#![allow(warnings)]

use std::default::Default as STDefault;
use std::hash::Hash;
use std::time::Duration;

use compose::foundation::background::BackgroundModifier;
use compose::foundation::bridge::platform_compose_view::MacOSComposeView;
use compose::foundation::composer::{Composer, ScopeUpdateScopeHelper};
use compose::foundation::desktop::window::DesktopWindow;
use compose::foundation::drawing::canvas_impl::new_canvas;
use compose::foundation::geometry::IntoDp;
use compose::foundation::layout::size_modifier::SizeModifier;
use compose::foundation::modifier::Modifier;
use compose::foundation::spacer::Spacer;
use compose::foundation::ui::align::Alignment;
use compose::foundation::ui::graphics::color::Color;
use compose::widgets::r#box::BoxLayout;
use compose::widgets::row::{Row, RowParams};
use minifb::{Key, KeyRepeat, Scale, ScaleMode, Window, WindowOptions};
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
        Spacer(Modifier.width(50.dp()));
        BoxLayout(Modifier.height(100.dp()).weight(row_scope, 1f32).vertical_align(row_scope, Alignment::CENTER_VERTICALLY).background(Color::YELLOW), |_| {});
    });
}

fn main() {
    DesktopWindow(Default::default(), || {
        test_widget();
    }, || {
        test_widget_move();
    });

    Composer::validate_group();
    Composer::debug_print();
    Composer::destroy();
    compose::foundation::memory::leak_token::validate_leak();

}
