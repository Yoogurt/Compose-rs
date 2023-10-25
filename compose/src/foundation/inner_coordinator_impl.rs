use std::cell::RefCell;
use std::mem::MaybeUninit;
use std::ops::DerefMut;
use std::rc::{Weak, Rc};
use crate::foundation::geometry::{IntOffset, IntSize};

use super::constraint::Constraint;
use super::inner_coodinator::InnerCoordinator;
use super::layout_node::{LayoutNodeLayoutDelegate, LayoutNode};
use super::layout_receiver::LayoutReceiver;
use super::layout_result::{Placeable, PlaceAction};
use super::look_ahead_capable_placeable::{LayoutNodeWrapperImpl, LayoutNodeWrapper};
use super::measurable::Measurable;
use super::measure_result::MeasureResult;
use super::measured::Measured;


fn error_measure_policy(_layout_receiver: LayoutReceiver, _children: &mut [&mut dyn Measurable], _constraint: &Constraint) -> MeasureResult {
    panic!("no measure policy provided")
}

impl InnerCoordinator {
    pub(crate) fn new() -> InnerCoordinator {
        InnerCoordinator {
            measure_policy: error_measure_policy,
            layout_node_layout_delegate: MaybeUninit::uninit(),
            layout_node_wrapper_impl: LayoutNodeWrapperImpl::new(),
        }
    }

    pub(crate) fn attach(&mut self, layout_node_layout_delegate: Rc<RefCell<LayoutNodeLayoutDelegate>>) {
        self.layout_node_layout_delegate = MaybeUninit::new(layout_node_layout_delegate);
    }

    pub(crate) fn handle_measured_result(&mut self, measure_result: MeasureResult) {
        dbg!(&measure_result);
        // self.set_measured_size(measure_result);
    }
}

impl Measurable for InnerCoordinator {
    fn measure(&mut self, constraint: &Constraint) -> &mut dyn Placeable {
        let measure_policy = self.measure_policy;
        let measure_result = {
            let children = &unsafe {self.layout_node_layout_delegate.assume_init_mut()}.borrow_mut().children;

            let children_rc = children.iter().map(|child| {
                child.borrow_mut().layout_node_layout_delegate.clone()
            }).collect::<Vec<_>>();

            let mut children_ref_mut = children_rc.iter().map(|child| {
                child.borrow_mut()
            }).collect::<Vec<_>>();

            let mut children_ref_mut = children_ref_mut.iter_mut().map(|child| {
                child.deref_mut().as_measurable_mut()
            }).collect::<Vec<_>>();

            let mut children_dyn_measurable = children_ref_mut.iter_mut().map(|child| {
                child.deref_mut()
            }).collect::<Vec<_>>();

            let layout_receiver = LayoutReceiver::new();
            measure_policy(layout_receiver, &mut children_dyn_measurable[..], constraint)
        };

        self.handle_measured_result(measure_result);
        self.layout_node_wrapper_impl.deref_mut()
    }
}

impl Placeable for InnerCoordinator {
    fn get_width(&self) -> usize {
        self.layout_node_wrapper_impl.get_width()
    }

    fn get_height(&self) -> usize {
        self.layout_node_wrapper_impl.get_height()
    }

    fn set_measured_size(&mut self, size: IntSize) {
       self.layout_node_wrapper_impl.set_measured_size(size)
    }

    fn get_measured_size(&self) -> IntSize {
        self.layout_node_wrapper_impl.get_measured_size()
    }

    fn get_measurement_constraint(&self) -> &Constraint {
        self.layout_node_wrapper_impl.get_measurement_constraint()
    }

    fn place_at(&mut self, position: IntOffset, z_index: f32, place_action: PlaceAction) {
        self.layout_node_wrapper_impl.place_at(position, z_index, place_action)
    }

    fn set_measurement_constraint(&mut self, constraint: &Constraint) {
        self.layout_node_wrapper_impl.set_measurement_constraint(constraint)
    }

    fn perfroming_measure(&mut self, constraint: &Constraint, block: MeasureAction) -> &dyn Placeable {
        self.layout_node_wrapper_impl.perfroming_measure(constraint, block)
    }
}

impl Measured for InnerCoordinator {
    fn get_measured_width(&self) -> usize {
        self.layout_node_wrapper_impl.get_measured_width()
    }

    fn get_measured_height(&self) -> usize {
        self.layout_node_wrapper_impl.get_measured_height()
    }
}

impl LayoutNodeWrapper for InnerCoordinator {
    fn layout_node(&self) -> Weak<RefCell<LayoutNode>> {
        self.layout_node_wrapper_impl.layout_node()
    }

    fn attach(&mut self, layout_node: Weak<RefCell<LayoutNode>>) {
        self.layout_node_wrapper_impl.attach(layout_node);
    }
}