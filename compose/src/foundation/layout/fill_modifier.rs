use crate::foundation::measure_scope::MeasureScopeLayoutAction;
use crate::foundation::modifier::ModifierNodeElement;
use crate::foundation::modifier::ModifierNodeImpl;
use auto_delegate::Delegate;
use compose_foundation_macro::ModifierElement;
use crate::foundation::constraint::Constraints;
use crate::foundation::measurable::Measurable;
use crate::foundation::measure_result::MeasureResult;
use crate::foundation::measure_scope::MeasureScope;
use crate::foundation::modifier::Modifier;
use crate::foundation::modifier_node::LayoutModifierNode;

impl Modifier {
    pub fn fill_max_size(self, fraction: Option<f32>) -> Modifier {
        self.then(fill_element(Direction::Both, fraction.unwrap_or(1.0)))
    }

    pub fn fill_max_width(self, fraction: Option<f32>) -> Modifier {
        self.then(fill_element(Direction::Horizontal, fraction.unwrap_or(1.0)))
    }

    pub fn fill_max_height(self, fraction: Option<f32>) -> Modifier {
        self.then(fill_element(Direction::Vertical, fraction.unwrap_or(1.0)))
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Vertical,
    Horizontal,
    Both,
}

fn fill_element(direction: Direction, fraction: f32) -> Modifier {
    ModifierNodeElement(
        "FillElement",
        move || {
            FillModifierNode {
                direction,
                fraction,
                node_impl: ModifierNodeImpl::default(),
            }
        },
        move |modifier: &mut FillModifierNode| {
            modifier.direction = direction;
            modifier.fraction = fraction;
        },
    )
}

#[derive(Debug, Delegate, ModifierElement)]
#[Impl(Layout)]
struct FillModifierNode {
    direction: Direction,
    fraction: f32,

    #[to(ModifierNode)]
    node_impl: ModifierNodeImpl,
}

impl LayoutModifierNode for FillModifierNode {
    fn measure(&self, measure_scope: &mut dyn MeasureScope, measurable: &mut dyn Measurable, constraint: &Constraints) -> MeasureResult {
        let min_width: usize;
        let max_width: usize;

        if constraint.has_bounded_width() && self.direction != Direction::Vertical {
            let width = (constraint.max_width as f32 * self.fraction).round() as usize;
            min_width = width;
            max_width = width;
        } else {
            min_width = constraint.min_width;
            max_width = constraint.max_width;
        }

        let min_height: usize;
        let max_height: usize;

        if constraint.has_bounded_height() && self.direction != Direction::Horizontal {
            let height = (constraint.max_height as f32 * self.fraction).round() as usize;
            min_height = height;
            max_height = height;
        } else {
            min_height = constraint.min_height;
            max_height = constraint.max_height;
        }

        let (size, placeable) = measurable.measure(&Constraints::new(min_width..=max_width, min_height..=max_height));

        measure_scope.layout(size, move |placement_scope| {
            placement_scope.place_relative(&placeable, 0, 0);
        })
    }
}