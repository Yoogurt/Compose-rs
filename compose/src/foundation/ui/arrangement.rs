use std::fmt::Debug;
use crate::foundation::geometry::{Density, Dp, IntoDp};
use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::utils::box_wrapper::WrapWithBox;

pub struct Arrangement;

pub type ArrangementHorizontal<'a> = &'a dyn ArrangementHorizontalTrait;
pub type ArrangementVertical<'a> = &'a dyn ArrangementVerticalTrait;
pub type ArrangementHorizontalOrVertical<'a> = &'a dyn ArrangementHorizontalOrVerticalTrait;

type ArrangementHorizontalStatic = ArrangementHorizontal<'static>;
type ArrangementVerticalStatic = ArrangementVertical<'static>;
type ArrangementHorizontalOrVerticalStatic = ArrangementHorizontalOrVertical<'static>;

impl Arrangement {
    pub const START: ArrangementHorizontalStatic = &ArrangementHorizontalImpl {
        spacing: Dp::ZERO,
        arrangement_horizontal_impl: ArrangementHorizontalStart::arrange,
        tag: "ArrangementHorizontalStart",
    };
    pub const END: ArrangementHorizontalStatic = &ArrangementHorizontalImpl {
        spacing: Dp::ZERO,
        arrangement_horizontal_impl: ArrangementHorizontalEnd::arrange,
        tag: "ArrangementHorizontalEnd",
    };

    pub const TOP: ArrangementVerticalStatic = &ArrangementVerticalImpl {
        spacing: Dp::ZERO,
        arrangement_vertical_impl: ArrangementVerticalTop::arrange,
        tag: "ArrangementVerticalTop",
    };

    pub const BOTTOM: ArrangementVerticalStatic = &ArrangementVerticalImpl {
        spacing: Dp::ZERO,
        arrangement_vertical_impl: ArrangementVerticalBottom::arrange,
        tag: "ArrangementVerticalBottom",
    };

    pub const CENTER: ArrangementHorizontalOrVerticalStatic = &ArrangementHorizontalOrVerticalImpl {
        spacing: Dp::ZERO,
        arrangement_impl: ArrangementCenter::arrange,
        tag: "ArrangementCenter",
    };
}

pub trait ArrangementHorizontalTrait {
    fn arrange(&self, density: Density, total_size: usize, sizes: &[usize], layout_direction: LayoutDirection) -> Vec<i32>;
}

struct ArrangementHorizontalImpl {
    spacing: Dp,
    arrangement_horizontal_impl: fn(Density, usize, &[usize], LayoutDirection) -> Vec<i32>,
    tag: &'static str,
}

impl ArrangementHorizontalTrait for ArrangementHorizontalImpl {
    fn arrange(&self, density: Density, total_size: usize, sizes: &[usize], layout_direction: LayoutDirection) -> Vec<i32> {
        (self.arrangement_horizontal_impl)(density, total_size, sizes, layout_direction)
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

pub trait ArrangementHorizontalOrVerticalTrait: ArrangementHorizontalTrait + ArrangementVerticalTrait {}

struct ArrangementVerticalBottom;

impl ArrangementVerticalBottom {
    fn arrange(density: Density, total_size: usize, sizes: &[usize]) -> Vec<i32> {
        place_right_or_bottom(total_size, sizes, false)
    }
}

struct ArrangementHorizontalOrVerticalImpl {
    spacing: Dp,
    arrangement_impl: fn(Density, usize, &[usize], LayoutDirection) -> Vec<i32>,
    tag: &'static str,
}

impl ArrangementHorizontalTrait for ArrangementHorizontalOrVerticalImpl {
    fn arrange(&self, density: Density, total_size: usize, sizes: &[usize], layout_direction: LayoutDirection) -> Vec<i32> {
        (self.arrangement_impl)(density, total_size, sizes, layout_direction)
    }
}

impl ArrangementVerticalTrait for ArrangementHorizontalOrVerticalImpl {
    fn arrange(&self, density: Density, total_size: usize, sizes: &[usize]) -> Vec<i32> {
        (self.arrangement_impl)(density, total_size, sizes, LayoutDirection::Ltr)
    }
}

fn place_center(density: Density, total_size: usize, sizes: &[usize], reverse_input: bool) -> Vec<i32>{
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

impl ArrangementHorizontalOrVerticalTrait for ArrangementHorizontalOrVerticalImpl {}

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