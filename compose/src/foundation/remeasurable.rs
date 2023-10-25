pub trait Remeasurable {
    fn remeasure(&mut self, constraint: &Constraint) -> bool;
}