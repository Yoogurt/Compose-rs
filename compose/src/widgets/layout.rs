use compose_macro::Compose;

use crate::foundation::composer::Composer;
use crate::foundation::measurable::{MultiChildrenMeasurePolicy, SingleChildMeasurePolicy};
use crate as compose;
use crate::foundation::modifier::Modifier;
use crate::foundation::utils::box_wrapper::WrapWithBox;

impl Modifier {
    pub fn layout(self, measure_policy: SingleChildMeasurePolicy) -> Modifier {
        self.then(Self::layout_element(measure_policy))
    }

    fn layout_element(measure_policy: SingleChildMeasurePolicy) -> Modifier {
        Modifier::ModifierNodeElement {
            create: Box::new(|| {
                todo!()
            }),
            update: Box::new(|_| {

            })
        }
    }
}

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