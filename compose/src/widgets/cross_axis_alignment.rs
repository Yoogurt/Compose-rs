use std::borrow::Borrow;
use std::fmt::Debug;

use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::placeable::Placeable;
use crate::foundation::ui::align::{AlignmentHorizontal, AlignmentVertical};

#[derive(Debug, Clone, Copy)]
pub enum CrossAxisAlignment {
    START,
    CENTER,
    END,
    HORIZONTAL(AlignmentHorizontal),
    VERTICAL(AlignmentVertical),
}

impl CrossAxisAlignment {
    pub fn align(&self, size: usize, layout_direction: LayoutDirection, placeable: &dyn Placeable, before_cross_axis_aligment_line: i32) -> i32 {
        match self {
            Self::START => start_align(size, layout_direction, placeable, before_cross_axis_aligment_line),
            Self::CENTER => center_align(size, layout_direction, placeable, before_cross_axis_aligment_line),
            Self::END => end_align(size, layout_direction, placeable, before_cross_axis_aligment_line),
            Self::HORIZONTAL(alignment_horizontal) => horizontal_align(alignment_horizontal, size, layout_direction, placeable, before_cross_axis_aligment_line),
            Self::VERTICAL(alignment_vertical) => vertical_align(alignment_vertical, size, layout_direction, placeable, before_cross_axis_aligment_line),
        }
    }
}

fn center_align(size: usize, layout_direction: LayoutDirection, placeable: &dyn Placeable, before_cross_axis_aligment_line: i32) -> i32 {
    size as i32 / 2
}

fn start_align(size: usize, layout_direction: LayoutDirection, placeable: &dyn Placeable, before_cross_axis_aligment_line: i32) -> i32 {
    match layout_direction {
        LayoutDirection::Ltr => 0,
        LayoutDirection::Rtl => size as i32,
    }
}

fn end_align(size: usize, layout_direction: LayoutDirection, placeable: &dyn Placeable, before_cross_axis_aligment_line: i32) -> i32 {
    match layout_direction {
        LayoutDirection::Ltr => size as i32,
        LayoutDirection::Rtl => 0,
    }
}

fn horizontal_align(horizontal: &AlignmentHorizontal, size: usize, layout_direction: LayoutDirection, placeable: &dyn Placeable, before_cross_axis_aligment_line: i32) -> i32 {
    horizontal.align(0, size, layout_direction)
}

fn vertical_align(vertical: &AlignmentVertical, size: usize, layout_direction: LayoutDirection, placeable: &dyn Placeable, before_cross_axis_aligment_line: i32) -> i32 {
    vertical.align(0, size)
}