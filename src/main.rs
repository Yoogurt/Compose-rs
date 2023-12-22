#![allow(warnings)]
use std::default::{Default as STDefault};
use std::hash::Hash;

use compose::foundation::composer::{Composer, ScopeUpdateScopeHelper};
use compose::foundation::desktop::window::{DesktopWindow, DesktopWindowOption};
use compose::foundation::geometry::IntoDp;
use compose::foundation::layout::size_modifier::SizeModifier;
use compose::foundation::measure_scope::MeasureScopeLayoutAction;
use compose::foundation::modifier::Modifier;
use compose::foundation::ui::align::{Alignment, AlignmentHorizontal};
use compose::foundation::ui::graphics::color::Color;
use compose::foundation::ui::input::pointer_event::PointerEventPass;
use compose::widgets::column::{Column, ColumnParams};
use compose::widgets::r#box::BoxLayout;

fn test_widget() {
    Column(Modifier.pointer_input(|scope| {
        scope.await_pointer_event_scope(|scope| {
            async move {
                let event = scope.await_pointer_event(PointerEventPass::Main).await;
                dbg!(event);
            }
        });
    }).fill_max_size(None).background(Color::BLUE).graphics_layer(|scope| {
        scope.set_scale_x(1.5);
        scope.set_scale_y(0.3);
        scope.set_alpha(0.5);
    }), ColumnParams::default(), |scope| {
    });
}

fn test_widget_move() {
    BoxLayout(Modifier.padding_top(100.dp()).padding_start(50.dp()).width(200.dp()).height(200.dp()).background(Color::BLUE), |scope| {
        BoxLayout(Modifier.width(100.dp()).height(100.dp()).align(scope, Alignment::CENTER).background(Color::YELLOW), |_| {});
    });
}

fn main() {
    DesktopWindow(DesktopWindowOption::default(), || {
        test_widget();
    }, || {
        test_widget_move();
    });

    Composer::validate_group();
    Composer::destroy();
    compose::foundation::memory::leak_token::validate_leak();
}
