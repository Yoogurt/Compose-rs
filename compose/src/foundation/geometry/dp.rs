#[derive(Debug, Copy, Clone)]
pub struct Dp {
    value: f64
}

pub trait IntoDp {
    fn dp(self) -> Dp;
}