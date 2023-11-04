use super::constraint::Constraints;

pub trait Remeasurable {
    fn remeasure(&mut self, constraint: &Constraints) -> bool;
}

pub trait StatefulRemeasurable: Remeasurable {
    fn mark_remeasure_pending(&mut self);
}
