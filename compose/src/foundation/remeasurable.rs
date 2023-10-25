use super::constraint::Constraint;

pub trait Remeasurable {
    fn remeasure(&mut self, constraint: &Constraint) -> bool;
}