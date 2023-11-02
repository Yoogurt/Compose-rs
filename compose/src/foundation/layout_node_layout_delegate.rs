use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use crate::foundation::constraint::Constraint;
use crate::foundation::look_ahead_pass_delegate::LookaheadPassDelegate;
use crate::foundation::measurable::Measurable;
use crate::foundation::measure_pass_delegate::MeasurePassDelegate;
use crate::foundation::modifier_container::ModifierContainer;
use crate::foundation::node_chain::NodeChain;
use crate::foundation::remeasurable::Remeasurable;
use crate::foundation::usage_by_parent::UsageByParent;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;

#[derive(Debug)]
pub(crate) struct LayoutNodeLayoutDelegate {
    pub(crate) last_constraints : Option<Constraint>,
    pub(crate) nodes: Option<Rc<RefCell<NodeChain>>>,
    pub(crate) modifier_container: Rc<RefCell<ModifierContainer>>,
    pub(crate) measure_pass_delegate: Rc<RefCell<MeasurePassDelegate>>,
    pub(crate) lookahead_pass_delegate: Rc<RefCell<LookaheadPassDelegate>>,
    pub(crate) measure_pending: bool,
    pub(crate) layout_pending: bool,
}

impl LayoutNodeLayoutDelegate {
    pub(crate) fn new() -> Rc<RefCell<Self>> {
        LayoutNodeLayoutDelegate {
            last_constraints: None,
            modifier_container: ModifierContainer::new().wrap_with_rc_refcell(),
            nodes: None,
            measure_pass_delegate: MeasurePassDelegate::new().wrap_with_rc_refcell(),
            lookahead_pass_delegate: LookaheadPassDelegate::new().wrap_with_rc_refcell(),
            measure_pending: false,
            layout_pending: false,
        }
        .wrap_with_rc_refcell()
    }

    pub(crate) fn attach(
        &mut self,
        node_chain: Rc<RefCell<NodeChain>>,
        modifier_container: Rc<RefCell<ModifierContainer>>,
    ) {
        self.nodes = Some(node_chain.clone());
        self.modifier_container = modifier_container;
        self.measure_pass_delegate.borrow_mut().attach(node_chain);
    }

    pub(crate) fn as_measurable(&self) -> Ref<dyn Measurable> {
        self.measure_pass_delegate.borrow()
    }

    pub(crate) fn as_measurable_mut(&self) -> RefMut<dyn Measurable> {
        self.measure_pass_delegate.borrow_mut()
    }

    fn request_remeasure(&self) {}
    fn request_relayout(&self) {}

    pub fn remeasure(&mut self, mut constraint: Option<Constraint>) -> bool {
        if constraint.is_none() {
            constraint = self.last_constraints;
        }

        let size_changed = match constraint {
            Some(constraint) => self
                .measure_pass_delegate
                .clone()
                .borrow_mut()
                .remeasure(&constraint),
            None => false,
        };

        if size_changed {
            let parent = self.nodes.clone().unwrap().borrow().get_parent();
            if parent.strong_count() > 0 {
                match self.measure_pass_delegate.borrow().measured_by_parent {
                    UsageByParent::InMeasureBlock => {}
                    UsageByParent::InLayoutBlock => {}
                    _ => {}
                }
            }
        }

        size_changed
    }

    pub(crate) fn update_parent_data(&self) {
        if self.measure_pass_delegate.borrow_mut().update_parent_data() {}
    }
}