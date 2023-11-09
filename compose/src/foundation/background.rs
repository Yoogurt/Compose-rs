use auto_delegate::Delegate;
use crate::foundation::modifier::{Modifier, ModifierNodeImpl, NodeKind, NodeKindPatch};
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

fn background_element(color: Color) -> Modifier {
    Modifier::ModifierNodeElement {
        create: Box::new(move || {
            BackgroundNode {
                color,
                alpha: 1.0,
                node_impl: ModifierNodeImpl::default(),
            }.wrap_with_rc_refcell()
        }),
        update: Box::new(move |mut element| {
            let background_element = element.as_modifier_element_mut().as_any_mut().downcast_mut::<BackgroundNode>().unwrap();
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
#[Impl(DrawModifierNodeConverter)]
struct BackgroundNode {
    color: Color,
    alpha: f32,

    #[to(ModifierNode)]
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

impl DelegatableNode for BackgroundNode {}

impl DrawModifierNode for BackgroundNode {
    fn draw(&self, draw_scope: &mut dyn ContentDrawScope) {
        self.draw_rect(draw_scope);
        draw_scope.draw_content()
    }
}