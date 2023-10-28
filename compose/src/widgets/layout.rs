use compose_macro::Compose;

use crate::foundation::composer::Composer;
use crate::foundation::measurable::MultiChildrenMeasurePolicy;
use crate as compose;
use crate::foundation::modifier::Modifier;

#[Compose]
pub fn layout(modifier: Modifier, measure_policy: MultiChildrenMeasurePolicy, content: fn()) {
    let node = Composer::begin_node();
    {
        let node_mut = node.borrow_mut();
        node_mut.set_measure_policy(measure_policy);
        node_mut.set_modifier(modifier);
    }
    content();
    Composer::end_node(node);
}