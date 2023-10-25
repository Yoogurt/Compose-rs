use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

include!("rect.rs");
include!("offset.rs");
include!("size.rs");
include!("converter.rs");
include!("ranges.rs");

pub mod rect_impl;
pub mod offset_impl;
pub mod size_impl;