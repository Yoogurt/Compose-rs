pub trait Unsigned32Bit: Clone + Copy + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> {
    fn as_u32(self) -> u32;
    fn from_u32(data: u32) -> Self;
}

pub trait Signed32Bit: Unsigned32Bit + Neg<Output=Self> + Rem<Output=Self> {}
