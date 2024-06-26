use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::foundation::constraint::Constraints;
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::layout_node_container::LayoutNodeContainer;
use crate::foundation::layout_state::LayoutState;
use crate::foundation::measurable::Measurable;
use crate::foundation::measure_pass_delegate::MeasurePassDelegate;
use crate::foundation::node_chain::NodeChain;
use crate::foundation::remeasurable::Remeasurable;
use crate::foundation::usage_by_parent::UsageByParent;
use crate::foundation::utils::option_extension::OptionThen;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;

#[derive(Debug)]
pub(crate) struct LayoutNodeLayoutDelegate {
    pub(crate) debug_label: String,
    pub(crate) last_constraints: Option<Constraints>,
    pub(crate) nodes: Option<Rc<RefCell<NodeChain>>>,
    pub(crate) modifier_container: Rc<RefCell<LayoutNodeContainer>>,
    pub(crate) measure_pass_delegate: Rc<RefCell<MeasurePassDelegate>>,
    identify: u32,
}

impl LayoutNodeLayoutDelegate {
    pub(crate) fn new() -> Rc<RefCell<Self>> {
        LayoutNodeLayoutDelegate {
            debug_label: "".to_string(),
            last_constraints: None,
            modifier_container: LayoutNodeContainer::new().wrap_with_rc_refcell(),
            nodes: None,
            measure_pass_delegate: MeasurePassDelegate::new(),
            identify: 0,
        }.wrap_with_rc_refcell()
    }

    pub(crate) fn attach(
        &mut self,
        identify: u32,
        node_chain: &Rc<RefCell<NodeChain>>,
        layout_node_container: &Rc<RefCell<LayoutNodeContainer>>,
        layout_state: &Rc<RefCell<LayoutState>>,
    ) {
        self.identify = identify;
        self.nodes = Some(node_chain.clone());
        self.modifier_container = layout_node_container.clone();
        self.measure_pass_delegate
            .borrow_mut()
            .attach(identify, node_chain, layout_state, layout_node_container);
    }

    pub(crate) fn as_measurable(&self) -> Ref<dyn Measurable> {
        self.measure_pass_delegate.borrow()
    }

    pub(crate) fn as_measurable_mut(&self) -> RefMut<dyn Measurable> {
        self.measure_pass_delegate.borrow_mut()
    }

    fn request_remeasure(&self) {}
    fn request_relayout(&self) {}

    pub fn remeasure(&mut self, mut constraint: Option<Constraints>) -> bool {
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
            if parent.is_some() {
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
        if self.measure_pass_delegate.borrow_mut().update_parent_data() {
            _ = self.nodes.as_ref().unwrap().borrow().get_parent()
                .and_then(|parent| parent.upgrade())
                .then(|parent| parent.borrow().request_remeasure());
        }
    }

    pub(crate) fn update_parent_data_with_parent(&self, parent: Option<&LayoutNode>) {
        if self.measure_pass_delegate.borrow_mut().update_parent_data() {
            _ = parent.then(|parent| parent.request_remeasure());
        }
    }
}
