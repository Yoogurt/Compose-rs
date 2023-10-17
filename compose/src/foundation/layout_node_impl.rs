use super::{LayoutResult, LayoutNode};
use super::Measurable;

impl Default for LayoutNode {
    fn default() -> Self {
        LayoutNode {
            children: Default::default(),
            modifier: Default::default(),
        }
    }
}

impl Measurable for LayoutNode {
    fn perform_measure(&self) -> LayoutResult {
        todo!()
    }
}