#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dp {
    value: f64
}

pub trait IntoDp {
    fn dp(self) -> Dp;
}