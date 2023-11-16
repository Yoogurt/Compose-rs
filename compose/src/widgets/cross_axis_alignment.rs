use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::placeable::Placeable;

#[derive(Debug, Copy, Clone)]
pub struct CrossAxisAlignment {
    align_impl: fn(size: usize, layout_direction: LayoutDirection, placeable: &dyn Placeable, before_cross_axis_aligment_line: i32) -> i32,
    is_relative: bool,
    tag: &'static str,
}

impl CrossAxisAlignment {
    pub const CENTER: CrossAxisAlignment = CrossAxisAlignment {
        align_impl: center_cross_axis_aligment,
        is_relative: false,
        tag: "CenterCrossAxisAlignment",
    };

    pub const START: CrossAxisAlignment = CrossAxisAlignment {
        align_impl: start_cross_axis_aligment,
        is_relative: true,
        tag: "StartCrossAxisAlignment",
    };

    pub const END: CrossAxisAlignment = CrossAxisAlignment {
        align_impl: end_cross_axis_aligment,
        is_relative: true,
        tag: "EndCrossAxisAlignment",
    };
}

fn center_cross_axis_aligment(size: usize, layout_direction: LayoutDirection, placeable: &dyn Placeable, before_cross_axis_aligment_line: i32) -> i32 {
    size as i32 / 2
}

fn start_cross_axis_aligment(size: usize, layout_direction: LayoutDirection, placeable: &dyn Placeable, before_cross_axis_aligment_line: i32) -> i32 {
    match layout_direction {
        LayoutDirection::Ltr => 0,
        LayoutDirection::Rtl => size as i32,
    }
}

fn end_cross_axis_aligment(size: usize, layout_direction: LayoutDirection, placeable: &dyn Placeable, before_cross_axis_aligment_line: i32) -> i32 {
    match layout_direction {
        LayoutDirection::Ltr => size as i32,
        LayoutDirection::Rtl => 0,
    }
}
