use crate::foundation::measurable::SingleChildMeasurePolicy;
use crate::foundation::modifier::Modifier;

pub trait IntrinsicSizeModifier {
    fn width(self) -> Modifier;
    fn height(self) -> Modifier;
}

impl IntrinsicSizeModifier for Modifier {
    fn width(self) -> Modifier {
        self + Modifier::LayoutModifier {
            measure_policy: instrinsic_size_measure_policy()
        }
    }

    fn height(self) -> Modifier {
        todo!()
    }
}

fn instrinsic_size_measure_policy() -> SingleChildMeasurePolicy {
    todo!()
}