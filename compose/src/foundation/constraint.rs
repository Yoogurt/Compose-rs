#[derive(Debug, Clone, Copy)]
pub struct Constraint {
    pub min_width: usize,
    pub max_width: usize,
    pub min_height: usize,
    pub max_height: usize,
}
