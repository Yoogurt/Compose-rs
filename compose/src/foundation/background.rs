use crate::foundation::modifier::Modifier;
use crate::foundation::canvas::Canvas;
use crate::foundation::ui::draw::{ContentDrawScope, DrawModifierNode, DrawScope};
use crate::foundation::utils::box_wrapper::WrapWithBox;
use crate::foundation::ui::graphics::color::Color;
use skia_safe::{Rect};
use crate::foundation::geometry::{IntOffset, Offset};

pub trait BackgroundModifier {
    fn background(self, color: Color) -> Modifier;
}

impl BackgroundModifier for Modifier {
    fn background(self, color: Color) -> Modifier {
        self.then(Modifier::ModifierDrawElemet(Background {
            color,
            alpha: 1.0,
        }.wrap_with_box()))
    }
}

struct Background {
    color: Color,
    alpha: f32,
}

impl Background {
    fn draw_rect(&self, draw_scope: &mut Box<dyn ContentDrawScope>) {
        draw_scope.draw_rect(self.color, Offset::zero(), None, 1.0);
    }
}

impl DrawModifierNode for Background {
    fn draw(&self, mut draw_scope: Box<dyn ContentDrawScope>) {
        self.draw_rect(&mut draw_scope);
        draw_scope.draw_content()
    }
}