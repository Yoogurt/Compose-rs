use std::fmt::Debug;
use crate::foundation::geometry::{Density, Dp, IntoDp};
use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::utils::box_wrapper::WrapWithBox;

#[derive(Debug, Copy, Clone)]
pub enum ArrangementVertical {
    TOP,
    BOTTOM,
    CENTER,
}

#[derive(Debug, Copy, Clone)]
pub enum ArrangementHorizontal {
    START,
    END,
    CENTER,
}

impl ArrangementVertical {
    pub fn arrange(&self, density: Density, total_size: usize, sizes: &[usize], layout_direction: LayoutDirection) -> Vec<i32> {
        match self {
            Self::TOP => { ArrangementVerticalTop::arrange(density, total_size, sizes) }
            Self::BOTTOM => { ArrangementVerticalBottom::arrange(density, total_size, sizes) }
            Self::CENTER => { ArrangementCenter::arrange(density, total_size, sizes, layout_direction) }
        }
    }

    pub fn spacing(&self) -> Dp {
        Dp::ZERO
    }
}

impl ArrangementHorizontal {
    pub fn arrange(&self, density: Density, total_size: usize, sizes: &[usize], layout_direction: LayoutDirection) -> Vec<i32> {
        match self {
            Self::START => { ArrangementHorizontalStart::arrange(density, total_size, sizes, layout_direction) }
            Self::END => { ArrangementHorizontalEnd::arrange(density, total_size, sizes, layout_direction) }
            Self::CENTER => { ArrangementCenter::arrange(density, total_size, sizes, layout_direction) }
        }
    }

    pub fn spacing(&self) -> Dp {
        Dp::ZERO
    }
}

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

#[derive(Debug)]
struct ArrangementHorizontalStart;

impl ArrangementHorizontalStart {
    fn arrange(density: Density, total_size: usize, sizes: &[usize], layout_direction: LayoutDirection) -> Vec<i32> {
        match layout_direction {
            LayoutDirection::Ltr => {
                place_left_or_top(sizes, false)
            }
            LayoutDirection::Rtl => {
                place_right_or_bottom(total_size, sizes, true)
            }
        }
    }
}

#[derive(Debug)]
struct ArrangementHorizontalEnd;

impl ArrangementHorizontalEnd {
    fn arrange(density: Density, total_size: usize, sizes: &[usize], layout_direction: LayoutDirection) -> Vec<i32> {
        match layout_direction {
            LayoutDirection::Ltr => {
                place_right_or_bottom(total_size, sizes, false)
            }
            LayoutDirection::Rtl => {
                place_left_or_top(sizes, true)
            }
        }
    }
}

pub trait ArrangementVerticalTrait {
    fn arrange(&self, density: Density, total_size: usize, sizes: &[usize]) -> Vec<i32>;
}

struct ArrangementVerticalImpl {
    spacing: Dp,
    arrangement_vertical_impl: fn(Density, usize, &[usize]) -> Vec<i32>,
    tag: &'static str,
}

impl ArrangementVerticalTrait for ArrangementVerticalImpl {
    fn arrange(&self, density: Density, total_size: usize, sizes: &[usize]) -> Vec<i32> {
        (self.arrangement_vertical_impl)(density, total_size, sizes)
    }
}

#[derive(Debug)]
struct ArrangementVerticalTop;

impl ArrangementVerticalTop {
    fn arrange(density: Density, total_size: usize, sizes: &[usize]) -> Vec<i32> {
        place_left_or_top(sizes, false)
    }
}

struct ArrangementVerticalBottom;

impl ArrangementVerticalBottom {
    fn arrange(density: Density, total_size: usize, sizes: &[usize]) -> Vec<i32> {
        place_right_or_bottom(total_size, sizes, false)
    }
}

//
// pub trait ArrangementHorizontalOrVerticalTrait: ArrangementHorizontalTrait + ArrangementVerticalTrait {}
//
// struct ArrangementHorizontalOrVerticalImpl {
//     spacing: Dp,
//     arrangement_impl: fn(Density, usize, &[usize], LayoutDirection) -> Vec<i32>,
//     tag: &'static str,
// }
//
// impl ArrangementHorizontalTrait for ArrangementHorizontalOrVerticalImpl {
//     fn arrange(&self, density: Density, total_size: usize, sizes: &[usize], layout_direction: LayoutDirection) -> Vec<i32> {
//         (self.arrangement_impl)(density, total_size, sizes, layout_direction)
//     }
// }
//
// impl ArrangementVerticalTrait for ArrangementHorizontalOrVerticalImpl {
//     fn arrange(&self, density: Density, total_size: usize, sizes: &[usize]) -> Vec<i32> {
//         (self.arrangement_impl)(density, total_size, sizes, LayoutDirection::Ltr)
//     }
// }

// impl ArrangementHorizontalOrVerticalTrait for ArrangementHorizontalOrVerticalImpl {}
fn place_center(density: Density, total_size: usize, sizes: &[usize], reverse_input: bool) -> Vec<i32> {
    let consumed_size: usize = sizes.iter().sum();
    let current = (total_size - consumed_size) / 2;

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


struct ArrangementCenter;

impl ArrangementCenter {
    fn arrange(density: Density, total_size: usize, sizes: &[usize], layout_direction: LayoutDirection) -> Vec<i32> {
        match layout_direction {
            LayoutDirection::Ltr => {
                place_center(density, total_size, sizes, false)
            }
            LayoutDirection::Rtl => {
                place_center(density, total_size, sizes, true)
            }
        }
    }
}