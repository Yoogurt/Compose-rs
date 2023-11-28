#![allow(warnings)]

use std::default::Default as STDefault;
use std::hash::Hash;

use compose::foundation::background::BackgroundModifier;
use compose::foundation::composer::{Composer, ScopeUpdateScopeHelper};
use compose::foundation::desktop::window::DesktopWindow;
use compose::foundation::geometry::IntoDp;
use compose::foundation::layout::size_modifier::SizeModifier;
use compose::foundation::modifier::Modifier;
use compose::foundation::spacer::Spacer;
use compose::foundation::ui::align::{Alignment, AlignmentHorizontal};
use compose::foundation::ui::graphics::color::Color;
use compose::widgets::r#box::BoxLayout;
use compose::widgets::row::{Row, RowParams};
use minifb::{Key, KeyRepeat, Scale, ScaleMode, Window, WindowOptions};
use skia_safe::{AlphaType, ColorSpace, ColorType, ImageInfo, surfaces,
};

fn test_widget() {
    BoxLayout(Modifier.padding_top(100.dp()).padding_start(50.dp()).width(200.dp()).height(200.dp()).background(Color::BLUE), |scope| {
        BoxLayout(Modifier.width(100.dp()).height(100.dp()).align(scope, Alignment::CENTER).background(Color::YELLOW), |_| {});
        BoxLayout(Modifier.width(50.dp()).height(50.dp()).align(scope, Alignment::CENTER).background(Color::GREEN), |_| {});
    });
}

fn test_widget_move() {
    BoxLayout(Modifier.padding_top(100.dp()).padding_start(50.dp()).width(200.dp()).height(200.dp()).background(Color::BLUE), |scope| {
        BoxLayout(Modifier.width(100.dp()).height(100.dp()).align(scope, Alignment::CENTER).background(Color::YELLOW), |_| {});
    });
}

fn main() {
    DesktopWindow(Default::default(), || {
        test_widget();
    }, || {
        test_widget_move();
    });

    Composer::validate_group();
    Composer::destroy();
    compose::foundation::memory::leak_token::validate_leak();
}
