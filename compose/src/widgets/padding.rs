use auto_delegate::Delegate;
use compose_foundation_macro::ModifierElement;
use crate::foundation::geometry::{Dp, IntoDp};
use crate::foundation::modifier::{Modifier, modifier_node_element_creator, modifier_node_element_updater, ModifierNodeImpl};
use crate::impl_node_kind_layout_node;
use crate::foundation::constraint::Constraints;
use crate::foundation::geometry::usize_extension::MayBeOverflowAdd;
use crate::foundation::measurable::Measurable;
use crate::foundation::measure_result::MeasureResult;
use crate::foundation::measure_scope::{MeasureScope, MeasureScopeLayoutAction};
use crate::foundation::modifier_node::LayoutModifierNode;

impl Modifier {
    pub fn padding_horizontal(self, horizontal: Dp) -> Modifier { self.padding(horizontal, 0.dp()) }

    pub fn padding_vertical(self, vertical: Dp) -> Modifier {
        self.padding(0.dp(), vertical)
    }

    pub fn padding(self, horizontal: Dp, vertical: Dp) -> Modifier {
        self.then(padding_element(horizontal, horizontal, vertical, vertical, true))
    }

    pub fn padding_start(self, start: Dp) -> Modifier {
        self.then(padding_element(start, 0.dp(), 0.dp(), 0.dp(), true))
    }

    pub fn padding_end(self, end: Dp) -> Modifier {
        self.then(padding_element(0.dp(), end, 0.dp(), 0.dp(), true))
    }

    pub fn padding_top(self, top: Dp) -> Modifier {
        self.then(padding_element(0.dp(), 0.dp(), top, 0.dp(), true))
    }

    pub fn padding_bottom(self, bottom: Dp) -> Modifier {
        self.then(padding_element(0.dp(), 0.dp(), 0.dp(), bottom, true))
    }

    pub fn padding_left(self, left: Dp) -> Modifier {
        self.then(padding_element(left, 0.dp(), 0.dp(), 0.dp(), false))
    }

    pub fn padding_right(self, right: Dp) -> Modifier {
        self.then(padding_element(0.dp(), right, 0.dp(), 0.dp(), false))
    }
}

#[derive(Delegate, Debug, ModifierElement)]
#[Impl(LayoutModifierNodeConverter)]
struct PaddingElement {
    start: Dp,
    end: Dp,
    top: Dp,
    bottom: Dp,
    rtl_aware: bool,
    #[to(ModifierNode)]
    node_impl: ModifierNodeImpl,
}
impl_node_kind_layout_node!(PaddingElement);

impl LayoutModifierNode for PaddingElement {
    fn measure(&self, measure_scope: &mut dyn MeasureScope, measurable: &mut dyn Measurable, constraint: &Constraints) -> MeasureResult {
        let density = measure_scope.get_density();

        let horizontal = self.start.round_to_px(density) + self.end.round_to_px(density);
        let vertical = self.top.round_to_px(density) + self.bottom.round_to_px(density);

        let (measure_result, placeable) = measurable.measure(&constraint.offset((-horizontal, -vertical).into()));

        let width = constraint.constrain_width(measure_result.width.add_signed(horizontal));
        let height = constraint.constrain_height(measure_result.height.add_signed(vertical));

        let start = self.start;
        let top = self.top;
        let rtl_aware = self.rtl_aware;
        measure_scope.layout((width, height).into(), move |placement_scope| {
            if rtl_aware {
                placement_scope.place_relative(placeable.borrow_mut(), start.round_to_px(density), top.round_to_px(density))
            } else {
                placement_scope.place(placeable.borrow_mut(), start.round_to_px(density), top.round_to_px(density))
            }
        })
    }
}

fn padding_element(start: Dp,
                   end: Dp,
                   top: Dp,
                   bottom: Dp,
                   rtl_aware: bool) -> Modifier {
    Modifier::ModifierNodeElement {
        create: modifier_node_element_creator(move || PaddingElement {
            start,
            end,
            top,
            bottom,
            rtl_aware,
            node_impl: Default::default(),
        }),
        update: modifier_node_element_updater(move |padding_element: &mut PaddingElement| {
            padding_element.start = start;
            padding_element.end = end;
            padding_element.top = top;
            padding_element.bottom = bottom;
            padding_element.rtl_aware = rtl_aware;
        }),
    }
}