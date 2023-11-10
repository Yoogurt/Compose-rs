use crate::foundation::measurable::SingleChildMeasurePolicy;
use crate::foundation::modifier::Modifier;

pub(crate) trait IntrinsicSizeModifier {
    fn width(self) -> Modifier;
    fn height(self) -> Modifier;
}

impl IntrinsicSizeModifier for Modifier {
    fn width(self) -> Modifier {
        todo!()
        // self + Modifier::LayoutElement {
        //     measure_policy: instrinsic_size_measure_policy()
        // }
    }

    fn height(self) -> Modifier {
        todo!()
    }
}

fn instrinsic_size_measure_policy() -> SingleChildMeasurePolicy {
    todo!()
}
