use std::fmt::Debug;
use crate::foundation::geometry::{Density, Dp, IntoDp};
use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::utils::box_wrapper::WrapWithBox;

pub struct Arrangement {}

impl Arrangement {
    fn start() -> &'static ArrangementHorizontal {
        todo!()
    }
}

pub struct ArrangementHorizontal {
    spacing: Dp,
    arrangement_horizontal_impl: fn(&ArrangementHorizontal, Density, usize, &[usize], LayoutDirection) -> Vec<i32>,
    tag: &'static str,
}

#[derive(Debug)]
struct ArrangementHorizontalStart {}

impl ArrangementHorizontalStart {
    fn place_left_or_top(sizes: &[usize], reverse_input: bool) -> Vec<i32> {
        let mut current = 0;

        let mut iter: Box<dyn DoubleEndedIterator<Item=(usize, &usize)>> = sizes.iter().enumerate().wrap_with_box();
        if reverse_input {
            iter = iter.rev().wrap_with_box();
        }

        iter.map(|(index, size)| {
            let result = current as i32;
            current += size;
            result
        }).collect()
    }

    fn place_right_or_bottom(total_size: usize, sizes: &[usize], reverse_input: bool) -> Vec<i32> {
        let consumed_size: usize = sizes.iter().sum();
        let current = total_size - consumed_size;

        let mut current = 0;
        let mut iter: Box<dyn DoubleEndedIterator<Item=(usize, &usize)>> = sizes.iter().enumerate().wrap_with_box();
        if reverse_input {
            iter = iter.rev().wrap_with_box();
        }

        iter.map(|(index, size)| {
            let result = current as i32;
            current += size;
            result
        }).collect()
    }

    fn arrange<const N: usize>(density: Density, total_size: usize, sizes: &[usize], layout_direction: LayoutDirection) -> Vec<i32> {
        match layout_direction {
            LayoutDirection::Ltr => {
                Self::place_left_or_top(sizes, false)
            }
            LayoutDirection::Rtl => {
                Self::place_right_or_bottom(total_size, sizes, false)
            }
        }
    }
}

pub trait ArrangementVertical: Debug {
    fn get_spacing(&self) -> Dp {
        0.dp()
    }

    fn arrange<const N: usize>(&self, density: Density, total_size: usize, sizes: &[usize; N]) -> [i32; N];
}