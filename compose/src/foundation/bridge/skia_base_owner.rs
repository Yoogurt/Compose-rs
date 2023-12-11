use crate::foundation::ui::input::pointer_event_type::PointerInputEvent;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

use crate::foundation::bridge::root_measure_policy::root_measure_policy;
use crate::foundation::canvas::Canvas;
use crate::foundation::composer::Composer;
use crate::foundation::constraint::Constraints;
use crate::foundation::geometry::{Density, IntOffset, IntRect, Offset};
use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::measure_and_layout_delegate::MeasureAndLayoutDelegate;
use crate::foundation::node::{GesstureOwner, Owner};
use crate::foundation::ui::input::pointer_input_event_processor::{PointerInputEventProcessor, PositionCalculator};
use crate::foundation::ui::input::process_result::ProcessResult;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;

struct SkiaBaseOwnerPositionCalculator;
impl PositionCalculator for SkiaBaseOwnerPositionCalculator {
    fn screen_to_local(&self, position_on_screen: Offset<f32>) -> Offset<f32> {
        position_on_screen
    }

    fn local_to_screen(&self, position_on_local: Offset<f32>) -> Offset<f32> {
        position_on_local
    }
}

pub struct SkiaBaseOwner {
    bound: IntRect,
    root: Rc<RefCell<LayoutNode>>,
    measure_and_layout_delegate: MeasureAndLayoutDelegate,
    pointer_input_event_processor: PointerInputEventProcessor,
    position_calculator: Rc<dyn PositionCalculator>,
}

impl Drop for SkiaBaseOwner {
    fn drop(&mut self) {
        Composer::detach_root_layout_node();
        self.detach()
    }
}

impl SkiaBaseOwner {
    pub fn new(bound: IntRect) -> Rc<RefCell<SkiaBaseOwner>> {
        let root = LayoutNode::new();

        let measure_and_layout_delegate = MeasureAndLayoutDelegate::new(root.clone());

        let mut result = SkiaBaseOwner {
            bound,
            root: root.clone(),
            measure_and_layout_delegate,
            pointer_input_event_processor: PointerInputEventProcessor::new(root.clone()),
            position_calculator: Rc::new(SkiaBaseOwnerPositionCalculator),
        };

        result
            .measure_and_layout_delegate
            .update_root_measure_policy(root_measure_policy());

        if !Composer::attach_root_layout_node(root.clone()) {
            panic!("unable to create multiple compose view in single thread");
        }

        let result = result.wrap_with_rc_refcell();

        Self::init(&result);
        result
    }

    pub fn update_bound(&mut self, bound: IntRect) {
        self.bound = bound;
    }

    pub fn is_in_bound(&self, point: IntOffset) -> bool {
        self.bound.contains(point)
    }

    fn init(this: &Rc<RefCell<Self>>) {
        let root = this.borrow().root.clone();
        let this_real_type = Rc::downgrade(this);
        let owner: Weak<RefCell<dyn Owner>> = this_real_type;
        root.borrow_mut().attach(None, owner);
    }

    fn detach(&mut self) {
        self.root.borrow_mut().detach();
    }

    pub fn set_content(&self, content: impl Fn()) {
        Composer::do_set_content(content);
    }

    pub fn no_insert_set_content(&self, content: impl Fn()) {
        Composer::do_compose_validate_structure(content);
    }

    pub fn dispatch_measure(&mut self, width: usize, height: usize) {
        let constraint = Constraints::new(0..=width, 0..=height);
        self.measure_and_layout_delegate
            .update_root_constraints(constraint);
        self.measure_and_layout_delegate.measure_only();
    }

    pub fn dispatch_layout(&mut self) {
        self.measure_and_layout_delegate.measure_and_layout();
    }

    pub fn dispatch_draw(&mut self, canvas: &mut dyn Canvas) {
        let draw_delegate = self.measure_and_layout_delegate.root.borrow().layout_node_draw_delegate.clone();
        draw_delegate.borrow_mut().draw(canvas);
    }
}

impl Owner for SkiaBaseOwner {
    fn get_root(&self) -> Rc<RefCell<LayoutNode>> {
        self.root.clone()
    }

    fn get_density(&self) -> Density {
        todo!()
    }

    fn get_layout_direction(&self) -> LayoutDirection {
        todo!()
    }

    fn on_request_relayout(&mut self, layout_node: Weak<RefCell<LayoutNode>>) {
        todo!()
    }

    fn on_attach(&self, layout_node: &LayoutNode) {}

    fn on_detach(&self, layout_node: &LayoutNode) {}
}

impl GesstureOwner for SkiaBaseOwner {
    fn process_pointer_input(&mut self, event: PointerInputEvent, is_in_bounds: bool) -> ProcessResult {
        let event_pointer_in_bounds = event.pointers.iter().all(|it| self.bound.contains(it.position.as_int_offset()));
        self.pointer_input_event_processor.process(event, self.position_calculator.clone().deref(), is_in_bounds && event_pointer_in_bounds)
    }
}