use compose_macro::Compose;

use crate::foundation::{layout_receiver::LayoutReceiver, measurable::Measurable, constraint::Constraint, measure_result::MeasureResult, modifier::Modifier};
use crate::{self as compose};
use crate::widgets::layout::layout;

#[macro_export]
macro_rules! Box {
    ( $modifier_expr:tt, $($fn_body:tt)* ) => {
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

fn box_measure_policy(layout_receiver: LayoutReceiver, measurable: &mut [&mut dyn Measurable], constraint: &Constraint) -> MeasureResult {
    let children_count = measurable.len();
    match children_count {
        0 => { layout_receiver.layout(constraint.min_width, constraint.min_height) }
        1 => {
            let placeable = measurable[0].measure(constraint);
            placeable.place_at((0,0).into(), 0.0, Box::new(|_| {}));
            todo!()
        }
        _ =>{
            todo!()
        }
    }
}

#[Compose]
pub fn box_internal(modifier: Modifier, content: fn()) {
    layout(modifier, box_measure_policy, content);
}