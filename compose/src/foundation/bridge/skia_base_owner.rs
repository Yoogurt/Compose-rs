use crate::foundation::ui::input::pointer_event_type::PointerInputEvent;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::foundation::bridge::root_measure_policy::root_measure_policy;
use crate::foundation::canvas::Canvas;
use crate::foundation::composer::Composer;
use crate::foundation::constraint::Constraints;
use crate::foundation::geometry::{Density, IntOffset, IntRect};
use crate::foundation::layout_direction::LayoutDirection;
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::measure_and_layout_delegate::MeasureAndLayoutDelegate;
use crate::foundation::node::{GesstureOwner, Owner};
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;

pub struct SkiaBaseOwner {
    bound: IntRect,
    root: Rc<RefCell<LayoutNode>>,
    measure_and_layout_delegate: MeasureAndLayoutDelegate,
}

impl Drop for SkiaBaseOwner {
    fn drop(&mut self) {
        Composer::detach_root_layout_node();
        self.detach()
    }
}

impl SkiaBaseOwner {
    pub fn new(bound: IntRect) -> Rc<RefCell<SkiaBaseOwner>> {
        let measure_and_layout_delegate = MeasureAndLayoutDelegate::new();

        let mut result = SkiaBaseOwner {
            bound,
            root: measure_and_layout_delegate.root.clone(),
            measure_and_layout_delegate,
        };

        result
            .measure_and_layout_delegate
            .update_root_measure_policy(root_measure_policy());

        if !Composer::attach_root_layout_node(result.measure_and_layout_delegate.root.clone()) {
            panic!("unable to create multiple compose view in single thread");
        }

        let result = result.wrap_with_rc_refcell();

        Self::init(&result);
        result
    }

    pub fn update_bound(&mut self, bound: IntRect) {
        // dbg!(&bound);
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
    fn process_pointer_input(&mut self, event: PointerInputEvent) {
    }
}