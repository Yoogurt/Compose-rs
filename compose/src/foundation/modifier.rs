use super::measurable::SingleChildMeasurePolicy;

pub const Modifier: Modifier = Modifier::Unit;

#[derive(Default)]
pub enum Modifier {
    #[default]
    Unit,
    LayoutModifier {
        measure_policy: SingleChildMeasurePolicy
    },
    Combined {
        left: Box<Modifier>,
        right: Box<Modifier>,
    },
}