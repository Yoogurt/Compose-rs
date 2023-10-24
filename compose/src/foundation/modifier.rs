pub const Modifier: Modifier = Modifier::Unit;

#[derive(Default, Debug, PartialEq)]
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