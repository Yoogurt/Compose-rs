#[macro_export]
macro_rules! Box {
    ( $modifier_expr:tt, $($fn_body:tt)* ) => {
        compose::widgets::box_internal($modifier_expr, || {
             $($fn_body)*
        });
    };

    ( $($fn_body:tt)* ) => {
        compose::widgets::box_internal(std::default::Default::default(), || {
             $($fn_body)*
        });
    };
}

fn box_measure_policy(measurable: &[&mut dyn Measurable], constraint: &Constraint) -> MeasureResult {
    MeasureResult::default()
}

#[Compose]
pub fn box_internal(modifier: Modifier, content: fn()) {
    layout(modifier, box_measure_policy, content);
}