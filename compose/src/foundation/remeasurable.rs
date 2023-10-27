use super::constraint::Constraint;

pub trait Remeasurable {
    fn remeasure(&mut self, constraint: &Constraint) -> bool;
}

pub trait StatefulRemeasurable : Remeasurable {
    fn mark_remeasure_pending(&mut self);
}