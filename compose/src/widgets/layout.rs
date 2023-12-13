use std::rc::Rc;
use compose_macro::Composable;

use crate as compose;
use crate::foundation::compose_node::ComposeNode;
use crate::foundation::measurable::MultiChildrenMeasurePolicy;
use crate::foundation::modifier::Modifier;

#[Composable]
pub fn Layout(
    modifier: Modifier,
    measure_policy: MultiChildrenMeasurePolicy,
    content: impl FnMut(),
) {
    let materialzed = modifier.materialize();

    ComposeNode(
        move |updater| {
            updater.set(measure_policy, |node, measure_policy| {
                node.set_measure_policy(measure_policy);
            });
            updater.set(materialzed, |node, modifier| {
                node.set_modifier(modifier);
            })
        },
        content,
    );
}
