use std::fmt::Debug;

use crate::foundation::geometry::{IntOffset, IntSize};
use crate::foundation::layout_direction::LayoutDirection;

pub type Alignment = AlignmentStruct;

#[derive(Copy, Clone)]
pub struct AlignmentStruct {
    alignment_impl: fn(size: IntSize, space: IntSize, bias: (f32, f32), layout_direction: LayoutDirection) -> IntOffset,
    horizontal_bias: f32,
    vertical_bias: f32,
    tag: &'static str,
}

impl Debug for AlignmentStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Alignment")
            .field("tag", &self.tag)
            .field("horizontal_bias", &self.horizontal_bias)
            .field("vertical_bias", &self.vertical_bias)
            .finish()
    }
}

impl Default for AlignmentStruct {
    fn default() -> Self {
        AlignmentStruct::CENTER
    }
}

impl AlignmentStruct {
    pub fn align(&self, size: IntSize, space: IntSize, layout_direction: LayoutDirection) -> IntOffset {
        (self.alignment_impl)(size, space, (self.horizontal_bias, self.vertical_bias), layout_direction)
    }

    pub const fn new(horizontal_bias: f32, vertical_bias: f32) -> AlignmentStruct {
        AlignmentStruct {
            alignment_impl: BiasAlignment::static_align,
            tag: "BiasAlignment",
            horizontal_bias,
            vertical_bias,
        }
    }

    const fn new_with_tag(horizontal_bias: f32, vertical_bias: f32, tag: &'static str) -> AlignmentStruct {
        AlignmentStruct {
            alignment_impl: BiasAlignment::static_align,
            tag,
            horizontal_bias,
            vertical_bias,
        }
    }

    pub const fn horizontal(horizontal_bias: f32) -> AlignmentHorizontal {
        AlignmentHorizontal {
            horizontal_impl: BiasAlignment::static_horizontal_align,
            horizontal_bias,
            tag: "BiasAlignmentHorizontal",
        }
    }

    const fn horizontal_with_tag(horizontal_bias: f32, tag: &'static str) -> AlignmentHorizontal {
        AlignmentHorizontal {
            horizontal_impl: BiasAlignment::static_horizontal_align,
            horizontal_bias,
            tag,
        }
    }

    pub const fn vertical(vertical_bias: f32) -> AlignmentVertical {
        AlignmentVertical {
            vertical_impl: BiasAlignment::static_vertical_align,
            vertical_bias,
            tag: "BiasAlignmentVertical",
        }
    }

    const fn vertical_with_tag(vertical_bias: f32, tag: &'static str) -> AlignmentVertical {
        AlignmentVertical {
            vertical_impl: BiasAlignment::static_vertical_align,
            vertical_bias,
            tag,
        }
    }

    pub const TOP_START: AlignmentStruct = AlignmentStruct::new_with_tag(-1.0, -1.0, "BiasAlignment(top_start)");
    pub const TOP_CENTER: AlignmentStruct = AlignmentStruct::new_with_tag(0.0, -1.0, "BiasAlignment(top_center)");
    pub const TOP_END: AlignmentStruct = AlignmentStruct::new_with_tag(1.0, -1.0, "BiasAlignment(top_end)");

    pub const CENTER_START: AlignmentStruct = AlignmentStruct::new_with_tag(-1.0, 0.0, "BiasAlignment(center_start)");
    pub const CENTER: AlignmentStruct = AlignmentStruct::new_with_tag(0.0, 0.0, "BiasAlignment(center)");
    pub const CENTER_END: AlignmentStruct = AlignmentStruct::new_with_tag(1.0, 0.0, "BiasAlignment(center_end)");

    pub const BOTTOM_START: AlignmentStruct = AlignmentStruct::new_with_tag(-1.0, 1.0, "BiasAlignment(bottom_start)");
    pub const BOTTOM_CENTER: AlignmentStruct = AlignmentStruct::new_with_tag(0.0, 1.0, "BiasAlignment(bottom_center)");
    pub const BOTTOM_END: AlignmentStruct = AlignmentStruct::new_with_tag(1.0, 1.0, "BiasAlignment(bottom_end)");

    pub const TOP: AlignmentVertical = AlignmentStruct::vertical_with_tag(-1.0, "BiasAlignmentVertical(top)");
    pub const CENTER_VERTICALLY: AlignmentVertical = AlignmentStruct::vertical_with_tag(0.0, "BiasAlignmentVertical(center_vertically)");
    pub const BOTTOM: AlignmentVertical = AlignmentStruct::vertical_with_tag(1.0, "BiasAlignmentVertical(bottom)");

    pub const START: AlignmentHorizontal = AlignmentStruct::horizontal_with_tag(-1.0, "BiasAlignmentHorizontal(start)");
    pub const CENTER_HORIZONTALLY: AlignmentHorizontal = AlignmentStruct::horizontal_with_tag(0.0, "BiasAlignmentHorizontal(center_horizontally)");
    pub const END: AlignmentHorizontal = AlignmentStruct::horizontal_with_tag(1.0, "BiasAlignmentHorizontal(end)");
}

#[derive(Copy, Clone)]
pub struct AlignmentHorizontal {
    horizontal_impl: fn(size: usize, space: usize, layout_direction: LayoutDirection, bias: f32) -> i32,
    horizontal_bias: f32,
    tag: &'static str,
}

impl Debug for AlignmentHorizontal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Horizontal")
            .field("tag", &self.tag)
            .field("horizontal_bias", &self.horizontal_bias)
            .finish()
    }
}

impl AlignmentHorizontal {
    pub fn align(&self, size: usize, space: usize, layout_direction: LayoutDirection) -> i32 {
        (self.horizontal_impl)(size, space, layout_direction, self.horizontal_bias)
    }
}

#[derive(Copy, Clone)]
pub struct AlignmentVertical {
    vertical_impl: fn(size: usize, space: usize, bias: f32) -> i32,
    vertical_bias: f32,
    tag: &'static str,
}

impl Debug for AlignmentVertical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vertical")
            .field("tag", &self.tag)
            .field("vertical_bias", &self.vertical_bias)
            .finish()
    }
}

impl AlignmentVertical {
    pub fn align(&self, size: usize, space: usize) -> i32 {
        (self.vertical_impl)(size, space, self.vertical_bias)
    }
}

struct BiasAlignment {}

trait AligmentTrait {
    fn static_align(size: IntSize,
                    space: IntSize,
                    bias: (f32, f32),
                    layout_direction: LayoutDirection) -> IntOffset;

    fn static_horizontal_align(size: usize, space: usize, layout_direction: LayoutDirection, bias: f32) -> i32;

    fn static_vertical_align(size: usize, space: usize, bias: f32) -> i32;
}

impl AligmentTrait for BiasAlignment {
    fn static_align(size: IntSize,
                    space: IntSize,
                    (horizontal_bias, vertical_bias): (f32, f32),
                    layout_direction: LayoutDirection) -> IntOffset {
        IntOffset::new(Self::static_horizontal_align(
            size.width,
            space.width,
            layout_direction,
            horizontal_bias,
        ), Self::static_vertical_align(
            size.height,
            space.height,
            vertical_bias,
        ), )
    }

    fn static_horizontal_align(size: usize, space: usize, layout_direction: LayoutDirection, bias: f32) -> i32 {
        let center = (space as i32 - size as i32) as f32 / 2f32;
        let resolved_bias = if layout_direction == LayoutDirection::Ltr {
            bias
        } else {
            -bias
        };
        (center * (1.0 + resolved_bias)).round() as i32
    }

    fn static_vertical_align(size: usize, space: usize, bias: f32) -> i32 {
        let center = (space as i32 - size as i32) as f32 / 2f32;
        (center * (1.0 + bias)).round() as i32
    }
}

struct BiasAbsoluteAligment {}

impl AligmentTrait for BiasAbsoluteAligment {
    fn static_align(size: IntSize, space: IntSize, bias: (f32, f32), layout_direction: LayoutDirection) -> IntOffset {
        todo!()
    }

    fn static_horizontal_align(size: usize, space: usize, layout_direction: LayoutDirection, bias: f32) -> i32 {
        todo!()
    }

    fn static_vertical_align(size: usize, space: usize, bias: f32) -> i32 {
        todo!()
    }
}