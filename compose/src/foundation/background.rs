use auto_delegate::Delegate;
use compose_foundation_macro::ModifierElement;

use crate::foundation::canvas::Canvas;
use crate::foundation::delegatable_node::DelegatableNode;
use crate::foundation::geometry::Offset;
use crate::foundation::modifier::{Modifier, modifier_node_element_creator, modifier_node_element_updater, ModifierNodeImpl, NodeKind, NodeKindPatch};
use crate::foundation::modifier_node::DrawModifierNode;
use crate::foundation::ui::draw::{ContentDrawScope, DrawScope};
use crate::foundation::ui::graphics::color::Color;

pub trait BackgroundModifier {
    fn background(self, color: Color) -> Modifier;
}

fn background_element(color: Color) -> Modifier {
    Modifier::ModifierNodeElement {
        create: modifier_node_element_creator(move || {
            BackgroundNode {
                color,
                alpha: 1.0,
                node_impl: ModifierNodeImpl::default(),
            }
        }),
        update: modifier_node_element_updater(move |background_element: &mut BackgroundNode| {
            background_element.color = color;
            background_element.alpha = 1.0;
        }),
    }
}

impl BackgroundModifier for Modifier {
    fn background(self, color: Color) -> Modifier {
        self.then(background_element(color))
    }
}

#[derive(Debug, ModifierElement, Delegate)]
#[Impl(Draw)]
struct BackgroundNode {
    color: Color,
    alpha: f32,

    #[to(ModifierNode, DelegatableNode)]
    node_impl: ModifierNodeImpl,
}

impl BackgroundNode {
    fn draw_rect(&self, draw_scope: &mut dyn ContentDrawScope) {
        draw_scope.draw_rect(self.color, Offset::zero(), Some(draw_scope.get_size()), 1.0);
    }
}

impl NodeKindPatch for BackgroundNode {
    fn get_node_kind(&self) -> NodeKind {
        NodeKind::Draw
    }
}

impl DrawModifierNode for BackgroundNode {
    fn draw(&self, draw_scope: &mut dyn ContentDrawScope) {
        self.draw_rect(draw_scope);
        draw_scope.draw_content()
    }
}