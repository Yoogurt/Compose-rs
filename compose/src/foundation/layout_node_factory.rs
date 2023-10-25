use super::constraint::Constraint;

pub(crate) struct LayoutNodeFactory {
    content: fn(),
    measure_policy: fn(constraint: Constraint),
}