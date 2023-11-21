use std::any::Any;
use std::cell::RefCell;
use std::cmp::max;
use std::ops::{Deref, DerefMut, RangeInclusive};
use std::rc::Rc;

use auto_delegate::Delegate;
use compose_foundation_macro::ModifierElement;

use crate::foundation::constraint::Constraints;
use crate::foundation::delegatable_node::{DelegatableKind, DelegatableNode};
use crate::foundation::geometry::{CoerceAtLeast, Density};
use crate::foundation::geometry::Dp;
use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::measurable::Measurable;
use crate::foundation::measure_scope::MeasureScope;
use crate::foundation::modifier::{ModifierNodeImpl, NodeKind, NodeKindPatch};
use crate::foundation::modifier_node::ParentDataModifierNode;
use crate::foundation::parent_data::ExtractParentData;
use crate::foundation::placeable::Placeable;
use crate::foundation::placement_scope::PlacementScope;
use crate::foundation::ui::align::{AlignmentHorizontal, AlignmentVertical};
use crate::foundation::ui::size_mode::SizeMode;
use crate::foundation::utils::option_extension::OptionalInstanceConverter;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;
use crate::impl_node_kind_parent_data;
use crate::widgets::cross_axis_alignment::CrossAxisAlignment;
use crate::widgets::orientation_independent_constrains::OrientationIndependentConstrains;

#[derive(Copy, Clone)]
pub(crate) enum LayoutOrientation {
    Horizontal,
    Vertical,
}

#[derive(Clone)]
pub(crate) struct RowColumnParentData {
    pub(crate) weight: f32,
    pub(crate) fill: bool,
    pub(crate) cross_axis_alignment: Option<CrossAxisAlignment>,
}

impl Default for RowColumnParentData {
    fn default() -> Self {
        Self { weight: 0f32, fill: true, cross_axis_alignment: None }
    }
}

#[derive(Debug, Delegate, ModifierElement)]
#[Impl(ParentDataModifierNodeConverter)]
pub(crate) struct HorizontalAlignModifier {
    pub(crate) alignment_horizontal: AlignmentHorizontal,
    #[to(ModifierNode)]
    node_impl: ModifierNodeImpl,
}

impl HorizontalAlignModifier {
    pub fn new(alignment_horizontal: AlignmentHorizontal) -> Self {
        Self {
            alignment_horizontal,
            node_impl: ModifierNodeImpl::default(),
        }
    }
}
impl_node_kind_parent_data!(HorizontalAlignModifier);

impl ParentDataModifierNode for HorizontalAlignModifier {
    fn modify_parent_data(&mut self, _: Density, parent_data: Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        let mut parent_data = parent_data.cast_or_init(|| {
            RowColumnParentData::default()
        });
        parent_data.cross_axis_alignment = Some(CrossAxisAlignment::HORIZONTAL(self.alignment_horizontal));
        Some(parent_data)
    }
}

#[derive(Debug, Delegate, ModifierElement)]
#[Impl(ParentDataModifierNodeConverter)]
pub(crate) struct VerticalAlignModifier {
    pub(crate) alignment_vertical: AlignmentVertical,
    #[to(ModifierNode)]
    node_impl: ModifierNodeImpl,
}
impl_node_kind_parent_data!(VerticalAlignModifier);

impl ParentDataModifierNode for VerticalAlignModifier {
    fn modify_parent_data(&mut self, _: Density, parent_data: Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        let mut parent_data = parent_data.cast_or_init(|| {
            RowColumnParentData::default()
        });
        parent_data.cross_axis_alignment = Some(CrossAxisAlignment::VERTICAL(self.alignment_vertical));
        Some(parent_data)
    }
}

impl VerticalAlignModifier {
    pub fn new(alignment_vertical: AlignmentVertical) -> Self {
        Self {
            alignment_vertical,
            node_impl: ModifierNodeImpl::default(),
        }
    }
}

#[derive(Debug, Delegate, ModifierElement)]
#[Impl(ParentDataModifierNodeConverter)]
pub(crate) struct LayoutWeightNode {
    pub(crate) weight: f32,
    pub(crate) fill: bool,

    #[to(ModifierNode)]
    node_impl: ModifierNodeImpl,
}
impl_node_kind_parent_data!(LayoutWeightNode);

impl ParentDataModifierNode for LayoutWeightNode {
    fn modify_parent_data(&mut self, _: Density, parent_data: Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        let mut parent_data = parent_data.cast_or_init(|| {
            RowColumnParentData::default()
        });
        parent_data.weight = self.weight;
        parent_data.fill = self.fill;

        Some(parent_data)
    }
}

impl LayoutWeightNode {
    pub fn new() -> Self {
        Self {
            weight: 0f32,
            fill: true,
            node_impl: ModifierNodeImpl::default(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct RowColumnMeasureHelper {
    pub(crate) orientation: LayoutOrientation,
    pub(crate) arrangement: fn(total_size: usize, size: &[usize], layout_direction: LayoutDirection, main_axis_positions: &mut [i32], density: Density) -> Vec<i32>,
    pub(crate) arrangement_spacing: Dp,
    pub(crate) cross_axis_size: SizeMode,
    pub(crate) cross_axis_alignment: CrossAxisAlignment,
}

pub(crate) struct RowColumnMeasureHelperResult {
    pub(crate) main_axis_size: usize,
    pub(crate) cross_axis_size: usize,
    pub(crate) range: RangeInclusive<usize>,
    pub(crate) main_axis_positions: Vec<i32>,
    pub(crate) before_cross_axis_alignment_line: i32,
}

pub(crate) trait RowColumnParentDataTrait {
    fn row_column_parent_data(&self) -> Option<&RowColumnParentData>;
}

impl<T> RowColumnParentDataTrait for T where T: ?Sized + Measurable {
    fn row_column_parent_data(&self) -> Option<&RowColumnParentData> {
        self.cast::<RowColumnParentData>()
    }
}

impl RowColumnMeasureHelper {
    fn main_axis_size(&self, placeable: &dyn Placeable) -> usize {
        match self.orientation {
            LayoutOrientation::Horizontal => { placeable.get_size().width }
            LayoutOrientation::Vertical => { placeable.get_size().height }
        }
    }

    fn cross_axis_size(&self, placeable: &dyn Placeable) -> usize {
        match self.orientation {
            LayoutOrientation::Horizontal => { placeable.get_size().height }
            LayoutOrientation::Vertical => { placeable.get_size().width }
        }
    }

    pub fn measure_without_placing(&self, measure_scope: &dyn MeasureScope,
                                   measurables: &mut [&mut dyn Measurable],
                                   parent_data: &[Option<RowColumnParentData>],
                                   placeables: &mut [Option<Rc<RefCell<dyn Placeable>>>],
                                   constraints: &Constraints,
                                   range: RangeInclusive<usize>) -> RowColumnMeasureHelperResult {
        let constraints = OrientationIndependentConstrains::new_with_orientation(*constraints, self.orientation);

        let arrangement_spacing_px = self.arrangement_spacing.round_to_px(measure_scope.get_density());

        let mut total_weight = 0f32;
        let mut fixed_space = 0usize;
        let mut cross_axis_space = 0usize;
        let mut weight_children_count = 0;

        let sub_size = range.end() - range.start() + 1;

        let mut space_after_last_no_weight = 0;
        for i in range.clone() {
            let child = &mut measurables[i];
            let parent_data = child.row_column_parent_data();
            let weight = parent_data.map(|parent_data| parent_data.weight).unwrap_or(0f32);

            if weight > 0f32 {
                total_weight += weight;
                weight_children_count += 1;
            } else {
                let main_axis_max = constraints.main_axis_max();
                let placeable = placeables[i].clone().unwrap_or_else(|| {
                    let constraints = constraints.copy(
                        0,
                        if main_axis_max == Constraints::INFINITE {
                            Constraints::INFINITE
                        } else {
                            (main_axis_max - fixed_space).coerce_at_least(0)
                        },
                        0,
                        constraints.cross_axis_max(),
                    ).to_box_constrains(self.orientation);

                    child.measure(&constraints).1
                });

                let placeable_main_axis_size = self.main_axis_size(placeable.borrow().deref());
                space_after_last_no_weight = (arrangement_spacing_px as usize).min(main_axis_max - fixed_space - placeable_main_axis_size);
                fixed_space += placeable_main_axis_size + space_after_last_no_weight;
                cross_axis_space = cross_axis_space.max(self.cross_axis_size(placeable.borrow().deref()));

                placeables[i] = Some(placeable);
            }
        }

        let mut weighted_space = 0;
        if weight_children_count == 0 {
            fixed_space -= space_after_last_no_weight;
        } else {
            let target_space = if total_weight > 0f32 && constraints.main_axis_max() != Constraints::INFINITE {
                constraints.main_axis_max()
            } else {
                constraints.main_axis_min()
            };

            let arrangement_space_total = arrangement_spacing_px as usize * (weight_children_count - 1);
            let remaining_to_target = target_space.checked_sub(fixed_space).unwrap_or(0).checked_sub(arrangement_space_total).unwrap_or(0);

            let weight_unit_space = if total_weight > 0f32 { remaining_to_target as f32 / total_weight } else { 0f32 };
            let mut remainder = remaining_to_target as i32 - parent_data[range.clone()].iter().fold(0, |acc, parent_data| {
                acc + (weight_unit_space * parent_data.as_ref().map(|parent_data| parent_data.weight).unwrap_or(0f32)).round() as i32
            });

            for i in range.clone() {
                let placeable = &mut placeables[i];
                if placeable.is_none() {
                    let child = &mut measurables[i];
                    let parent_data = parent_data[i].as_ref().unwrap();
                    let weight = parent_data.weight;
                    if weight <= 0f32 {
                        panic!("All weights <= 0 should have placeables")
                    }

                    let remainder_unit: i32 = if remainder > 0 {
                        1
                    } else if remainder < 0 {
                        -1
                    } else { 0 };

                    remainder -= remainder_unit;
                    let child_main_axis_size = ((weight_unit_space * weight + remainder_unit as f32).round() as usize).checked_add_signed(remainder_unit as isize).unwrap_or(0);

                    let (measure_result, placeable_result) = child.measure(&OrientationIndependentConstrains::new(
                        if parent_data.fill && child_main_axis_size != Constraints::INFINITE {
                            child_main_axis_size
                        } else {
                            0
                        },
                        child_main_axis_size,
                        0,
                        constraints.cross_axis_max(),
                    ).to_box_constrains(self.orientation));

                    {
                        let placeable = placeable_result.borrow();
                        weighted_space += self.main_axis_size(placeable.deref());
                        cross_axis_space = max(cross_axis_space, self.cross_axis_size(placeable.deref()));
                    }

                    *placeable = Some(placeable_result);
                }
            }
        }

        // align by

        let main_axis_layout_size = max((fixed_space + weighted_space).coerce_at_least(0), constraints.main_axis_min());
        let cross_axis_layout_size = if constraints.cross_axis_max() != Constraints::INFINITE && self.cross_axis_size == SizeMode::Expand {
            constraints.cross_axis_max()
        } else {
            max(cross_axis_space, max(constraints.cross_axis_min(), 0))
        };

        let mut main_axis_positions = vec![0; sub_size];
        let mut children_main_axis_size = (0..sub_size).map(|index| {
            self.main_axis_size(placeables[index + range.start()].as_ref().unwrap().borrow().deref())
        }).collect::<Vec<usize>>();

        RowColumnMeasureHelperResult {
            main_axis_size: main_axis_layout_size,
            cross_axis_size: cross_axis_layout_size,
            range,
            main_axis_positions: (self.arrangement)(main_axis_layout_size,
                                                    &children_main_axis_size,
                                                    measure_scope.get_layout_direction(),
                                                    &mut main_axis_positions,
                                                    measure_scope.get_density()),
            before_cross_axis_alignment_line: 0,
        }
    }

    fn get_cross_axis_position(&self, placeable: &dyn Placeable, row_column_parent_data: Option<&RowColumnParentData>, cross_axis_layout_size: usize, layout_direction: LayoutDirection, before_cross_axis_alignment_line: i32) -> i32 {
        let child_cross_alignment = row_column_parent_data.and_then(|parent_data| parent_data.cross_axis_alignment).unwrap_or(self.cross_axis_alignment);

        child_cross_alignment.align(cross_axis_layout_size - self.cross_axis_size(placeable), match self.orientation {
            LayoutOrientation::Horizontal => { LayoutDirection::Ltr }
            LayoutOrientation::Vertical => { layout_direction }
        }, placeable, before_cross_axis_alignment_line)
    }

    pub fn place_helper(&self,
                        placement_scope: &dyn PlacementScope,
                        measure_result: RowColumnMeasureHelperResult,
                        cross_axis_offset: i32,
                        layout_direction: LayoutDirection,
                        placeables: Vec<Option<Rc<RefCell<dyn Placeable>>>>,
                        parent_data: Vec<Option<RowColumnParentData>>,
    ) {
        let main_axis_positions = &measure_result.main_axis_positions;

        for i in measure_result.range.clone() {
            let mut placeable_rc = placeables[i].as_ref().unwrap().borrow_mut();
            let placeable = placeable_rc.deref_mut();
            let cross_axis_position = self.get_cross_axis_position(placeable, parent_data[i].as_ref(), measure_result.cross_axis_size, layout_direction, measure_result.before_cross_axis_alignment_line) + cross_axis_offset;

            match self.orientation {
                LayoutOrientation::Horizontal => {
                    placeable.place_at((main_axis_positions[i - measure_result.range.start()] as i32, cross_axis_position).into(), 0f32);
                }
                LayoutOrientation::Vertical => {
                    placeable.place_at((cross_axis_position, main_axis_positions[i - measure_result.range.start()] as i32).into(), 0f32);
                }
            }
        }
    }
}