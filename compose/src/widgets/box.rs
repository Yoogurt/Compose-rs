use compose_macro::Composable;

use crate::foundation::{measure_scope::MeasureScope, measurable::Measurable, constraint::Constraint, measure_result::MeasureResult, modifier::Modifier};
use crate::{self as compose};
use crate::foundation::utils::box_wrapper::WrapWithBox;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;

use crate::widgets::layout::Layout;

#[macro_export]
macro_rules! Box {
    ( $modifier_expr:expr, $($fn_body:tt)* ) => {
        compose::widgets::r#box::box_internal($modifier_expr, || {
             $($fn_body)*
        });
    };

    ( $($fn_body:tt)* ) => {
        compose::widgets::r#box::box_internal(std::default::Default::default(), || {
             $($fn_body)*
        });
    };
}

fn box_measure_policy(measure_scope: &mut dyn MeasureScope, measurable: &mut [&mut dyn Measurable], constraint: &Constraint) -> MeasureResult {
    let children_count = measurable.len();
    match children_count {
        0 => { measure_scope.layout(constraint.min_width, constraint.min_height, &mut |_| {}) }
        1 => {
            let placeable = measurable[0].measure(constraint);
            placeable.place_at((0,0).into(), 0.0);
            todo!()
        }
        _ =>{
            todo!()
        }
    }
}

#[Composable]
pub fn box_internal(modifier: Modifier, content: fn()) {
    Layout(modifier, box_measure_policy, content);
}