use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::placeable::Placeable;

#[derive(Debug, Copy, Clone)]
pub struct CrossAxisAlignment {
    align_impl: &'a dyn FnMut(usize, LayoutDirection, &dyn Placeable, i32) -> i32,
    is_relative: bool,
    tag: &'static str,
}

impl CrossAxisAlignment<'_> {
    pub fn CENTER() -> CrossAxisAlignment {
        Self {
            align_impl: &center_cross_axis_alignment,
            is_relative: false,
            tag: "CenterCrossAxisAlignment",
        }
    }

    pub fn START() -> CrossAxisAlignment {
        Self {
            align_impl: &start_cross_axis_alignment,
            is_relative: true,
            tag: "StartCrossAxisAlignment",
        }
    }

    pub fn END() -> CrossAxisAlignment {
        Self {
            align_impl: &end_cross_axis_alignment,
            is_relative: true,
            tag: "EndCrossAxisAlignment",
        }
    }

    // pub fn horizontal(horizontal: AlignmentHorizontal) -> Self {
    //     Self {
    //         align_impl: move |size: usize, layout_direction: LayoutDirection, placeable: &dyn Placeable, before_cross_axis_alignment_line: i32| {
    //             horizontal.align(0, size, layout_direction)
    //         },
    //         is_relative: false,
    //         tag: "HorizontalCrossAxisAlignment",
    //     }
    // }
}

fn center_cross_axis_alignment(size: usize, layout_direction: LayoutDirection, placeable: &dyn Placeable, before_cross_axis_aligment_line: i32) -> i32 {
    size as i32 / 2
}

fn start_cross_axis_alignment(size: usize, layout_direction: LayoutDirection, placeable: &dyn Placeable, before_cross_axis_aligment_line: i32) -> i32 {
    match layout_direction {
        LayoutDirection::Ltr => 0,
        LayoutDirection::Rtl => size as i32,
    }
}

fn end_cross_axis_alignment(size: usize, layout_direction: LayoutDirection, placeable: &dyn Placeable, before_cross_axis_aligment_line: i32) -> i32 {
    match layout_direction {
        LayoutDirection::Ltr => size as i32,
        LayoutDirection::Rtl => 0,
    }
}
