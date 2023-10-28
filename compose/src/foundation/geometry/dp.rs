#[derive(Debug, Copy, Clone)]
pub struct Dp {
    value: f32
}

pub trait IntoDp {
    fn dp(self) -> Dp;
}