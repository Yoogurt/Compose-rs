use crate::foundation::modifier::{Modifier, NodeKind, NodeKindPatch};
use crate::foundation::canvas::Canvas;
use crate::foundation::ui::draw::{ContentDrawScope, DrawModifierNode, DrawScope};
use crate::foundation::utils::box_wrapper::WrapWithBox;
use crate::foundation::ui::graphics::color::Color;
use skia_safe::{Rect};
use compose_foundation_macro::{AnyConverter, ModifierElement};
use crate::foundation::delegatable_node::DelegatableNode;
use crate::foundation::geometry::{IntOffset, Offset};
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;

pub trait BackgroundModifier {
    fn background(self, color: Color) -> Modifier;
}

impl BackgroundModifier for Modifier {
    fn background(self, color: Color) -> Modifier {
        self.then(Modifier::ModifierElement(Background {
            color,
            alpha: 1.0,
        }.wrap_with_rc_refcell()))
    }
}

#[derive(Debug, ModifierElement)]
struct Background {
    color: Color,
    alpha: f32,
}

impl Background {
    fn draw_rect(& self, draw_scope: &mut dyn ContentDrawScope) {
        draw_scope.draw_rect(self.color, Offset::zero(), None, 1.0);
    }
}

impl NodeKindPatch for Background {
    fn get_node_kind(& self) -> NodeKind {
        NodeKind::DrawModifierNode
    }
}

impl DelegatableNode for Background {}

impl DrawModifierNode for Background {
    fn draw(& self, draw_scope: &mut dyn ContentDrawScope) {
        self.draw_rect(draw_scope);
        draw_scope.draw_content()
    }
}