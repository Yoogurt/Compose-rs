use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use super::{Measurable, Canvas, Placeable, LayoutNode, LayoutNodeWrapperImpl, MultiChildrenMeasurePolicy, Modifier, Constraint, MeasureResult, InnerCoordinator, OuterCoordinator, LayoutNodeWrapper, LayoutState, LayoutReceiver, UsageByParent, NodeChain, LayoutNodeLayoutDelegate};

impl Deref for LayoutNode {
    type Target = NodeChain;

    fn deref(&self) -> &Self::Target {
        &self.node_chain
    }
}

impl DerefMut for LayoutNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node_chain
    }
}

impl LayoutNode {
    pub(crate) fn new() -> Rc<RefCell<Self>> {
        let mut node = LayoutNode {
            node_chain: NodeChain::new(),
            layout_node_layout_delegate: Rc::new(RefCell::new(LayoutNodeLayoutDelegate::new())),
            usage_by_parent: UsageByParent::NotUsed,
            layout_state: Default::default(),
        };

        let node = Rc::new(RefCell::new(node));
        {
            let layout_node_layout_delegate = node.borrow().layout_node_layout_delegate.clone();

            let mut node_mut = node.borrow_mut();
            node_mut.inner_placeable.borrow_mut().attach(layout_node_layout_delegate);
            let inner_layout_node_wrapper = node_mut.inner_placeable.clone();
            node_mut.outer_measurable_placeable.attach(inner_layout_node_wrapper);
        }

        node
    }

    pub fn set_modifier(&mut self, modifier: Modifier) {
        if self.modifier == modifier {
            return;
        }

        self.modifier = modifier;

        let outer_wrapper = self.modifier.fold_out::<Rc<RefCell<dyn LayoutNodeWrapper>>>(self.inner_placeable.clone(), &mut |modifier, to_wrap| {
            let mut wrapper = to_wrap;

            wrapper
        });
    }

    pub fn set_measure_policy(&self,
                              measure_policy: MultiChildrenMeasurePolicy) {
        self.inner_placeable.borrow_mut().measure_policy = measure_policy;
    }

    fn layout(width: usize, height: usize) -> MeasureResult {
        MeasureResult {
            width,
            height,
        }
    }

    pub(crate) fn adopt_child(&self, child: Rc<RefCell<LayoutNode>>) {
        // self.inner_placeable.borrow_mut().adopt_child(child);
    }

    pub fn remeasure(&mut self, constraint: &Constraint) -> bool {
        let outer_placeable = &mut self.outer_measurable_placeable;
        outer_placeable.remeasure(constraint)
    }

    fn draw(canvas: &dyn Canvas) {}
}

impl LayoutNodeLayoutDelegate {
    pub(crate) fn new() -> Self {
        LayoutNodeLayoutDelegate {
            children: vec![],
        }
    }
}

impl Measurable for LayoutNodeLayoutDelegate {
    fn measure(&mut self, constraint: &Constraint) -> &mut dyn Placeable {
        todo!()
        // self.outer_measurable_placeable.measure(constraint)
    }
}