use compose_macro::Compose;

use crate::foundation::composer::Composer;
use crate::foundation::measurable::{MultiChildrenMeasurePolicy, SingleChildMeasurePolicy};
use crate as compose;
use crate::foundation::compose_node::ComposeNode;
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
     ComposeNode(move |node| {
         node.set_measure_policy(measure_policy);
         node.set_modifier(modifier);
    });
}