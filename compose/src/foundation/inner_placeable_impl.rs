use std::cell::RefCell;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::rc::{Weak, Rc};
use crate::foundation::{Constraint, InnerPlaceable, MeasureResult, LayoutNode, LayoutNodeWrapper, LayoutNodeWrapperImpl, LayoutReceiver, Measurable, Measured, Placeable, PlaceAction};
use crate::foundation::geometry::{IntOffset, IntSize};
use crate::widgets::layout;

fn error_measure_policy(layout_receiver: LayoutReceiver, children: &mut [&mut dyn Measurable], constraint: &Constraint) -> MeasureResult {
    panic!("no measure policy provide")
}

impl InnerPlaceable {
    pub(crate) fn new() -> InnerPlaceable {
        InnerPlaceable {
            children: vec![],
            measure_policy: error_measure_policy,
            layout_node_wrapper_impl: LayoutNodeWrapperImpl::new(),
        }
    }

    pub(crate) fn handle_measured_result(&mut self, measure_result: MeasureResult) {
        dbg!(&measure_result);
        // self.set_measured_size(measure_result);
    }

    pub(crate) fn adopt_child(&mut self, child: Rc<RefCell<LayoutNode>>) {
        self.children.push(child);
    }
}

impl Measurable for InnerPlaceable {
    fn measure(&mut self, constraint: &Constraint) -> &mut dyn Placeable {
        let measure_policy = self.measure_policy;
        let measure_result = {
            let mut children_ref = self.children.iter().map(|child| {
                child.borrow_mut()
            }).collect::<Vec<_>>();

            let mut children = children_ref.iter_mut().map(|value| {
                value.deref_mut() as &mut dyn Measurable
            }).collect::<Vec<_>>();

            let mut layout_receiver = LayoutReceiver::new();
            measure_policy(layout_receiver, &mut children[..], constraint)
        };

        self.handle_measured_result(measure_result);
        self.layout_node_wrapper_impl.deref_mut()
    }
}

impl Placeable for InnerPlaceable {
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
}

impl Measured for InnerPlaceable {
    fn get_measured_width(&self) -> usize {
        self.layout_node_wrapper_impl.get_measured_width()
    }

    fn get_measured_height(&self) -> usize {
        self.layout_node_wrapper_impl.get_measured_height()
    }
}

impl LayoutNodeWrapper for InnerPlaceable {
    fn layout_node(&self) -> Weak<RefCell<LayoutNode>> {
        self.layout_node_wrapper_impl.layout_node()
    }

    fn attach(&mut self, layout_node: Weak<RefCell<LayoutNode>>) {
        self.layout_node_wrapper_impl.attach(layout_node);
    }
}