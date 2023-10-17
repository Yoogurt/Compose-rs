use crate::foundation::{Modifier, ComposeAttribute, LayoutNodeGuard};
use std::pin::Pin;

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


#[Compose(Mutable)]
pub fn box_internal(modifier: Modifier, child: fn()) {
    let node: LayoutNodeGuard<'_> = current_composer.begin_node();

    let node2 = node;
}