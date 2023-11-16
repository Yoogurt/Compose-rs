use std::any::Any;
use auto_delegate::Delegate;
use std::rc::Rc;
use std::cmp::max;
use std::rc::Weak;
use std::cell::{Ref, RefCell};
use std::ops::{Deref, RangeInclusive, RangeToInclusive};
use compose_foundation_macro::ModifierElement;
use crate::foundation::ui::size_mode::SizeMode;
use crate::foundation::constraint::Constraints;
use crate::foundation::delegatable_node::{DelegatableKind, DelegatableNode};
use crate::foundation::geometry::{CoerceAtLeast, Density};
use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::measure_scope::MeasureScope;
use crate::foundation::placeable::Placeable;
use crate::widgets::orientation_independent_constrains::OrientationIndependentConstrains;
use crate::foundation::geometry::Dp;
use crate::foundation::measurable::Measurable;
use crate::foundation::modifier::{ModifierNodeImpl, NodeKind, NodeKindPatch};
use crate::foundation::modifier_node::ParentDataModifierNode;
use crate::foundation::parent_data::ExtractParentData;
use crate::foundation::ui::align::AlignmentHorizontal;
use crate::foundation::utils::option_extension::OptionalInstanceConverter;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;
use crate::foundation::utils::self_reference::SelfReference;
use crate::widgets::cross_axis_alignment::CrossAxisAlignment;

#[derive(Copy, Clone)]
pub(crate) enum LayoutOrientation {
    Horizontal,
    Vertical,
}

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
    weak_self: Weak<RefCell<Self>>,
}

impl ParentDataModifierNode for HorizontalAlignModifier {
    fn modify_parent_data(&mut self, density: Density, parent_data: Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        let mut parent_data = parent_data.cast_or_init(|| {
            RowColumnParentData::default()
        });
        parent_data.cross_axis_alignment = Some(CrossAxisAlignment::horizontal(self.alignment_horizontal));
        Some(parent_data)
    }
}

impl SelfReference for HorizontalAlignModifier {
    fn get_self(&self) -> Weak<RefCell<Self>> {
        self.weak_self.clone()
    }
}

impl DelegatableNode for HorizontalAlignModifier {
    fn get_node(&self) -> DelegatableKind {
        DelegatableKind::This
    }
}

impl NodeKindPatch for HorizontalAlignModifier {
    fn get_node_kind(&self) -> NodeKind {
        NodeKind::ParentData
    }
}

impl HorizontalAlignModifier {
    pub fn new(alignment_horizontal: AlignmentHorizontal) -> Rc<RefCell<Self>> {
        let mut result = Self {
            alignment_horizontal,
            node_impl: ModifierNodeImpl::default(),
            weak_self: Weak::new(),
        }.wrap_with_rc_refcell();

        result.borrow_mut().weak_self = Rc::downgrade(&result);
        result
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
}

trait RowColumnParentDataTrait {
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
                                   placeables: &mut [Option<Rc<RefCell<dyn Placeable>>>],
                                   constraints: &Constraints,
                                   range: RangeInclusive<usize>) -> RowColumnMeasureHelperResult {
        let constraints = OrientationIndependentConstrains::new_with_orientation(*constraints, self.orientation);

        let arrangement_spacing_px = self.arrangement_spacing.round_to_px(measure_scope.get_density());

        let mut total_weight = 0f32;
        let mut fixed_space = 0usize;
        let mut cross_axis_space = 0usize;
        let mut weight_children_count = 0;

        let sub_size = range.end() - range.start();

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

        let weighted_space = 0;
        if weight_children_count == 0 {
            fixed_space -= space_after_last_no_weight;
        } else {
            todo!()
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
        }
    }
}