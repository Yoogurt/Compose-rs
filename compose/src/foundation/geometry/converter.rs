pub trait U64ConverterUnsigned: Clone + Copy + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> {
    fn as_u64(self) -> u64;
    fn from_u64(data: u64) -> Self;
}

pub trait U64ConverterSigned: U64ConverterUnsigned + Neg<Output=Self> + Rem<Output=Self> {}
