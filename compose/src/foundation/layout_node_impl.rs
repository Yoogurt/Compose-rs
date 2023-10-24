use std::cell::RefCell;
use std::ops::{DerefMut};
use std::rc::Rc;
use super::{Measurable, Canvas, Placeable, LayoutNode, LayoutNodeWrapperImpl, MultiChildrenMeasurePolicy, Modifier, Constraint, MeasureResult, InnerPlaceable, OuterMeasurePlaceable, LayoutNodeWrapper, LayoutState, LayoutReceiver, UsageByParent};

impl LayoutNode {
    pub(crate) fn new() -> Rc<RefCell<Self>> {
        let mut node = LayoutNode {
            usage_by_parent: UsageByParent::NotUsed,
            modifier: Modifier,
            parent_data: None,
            measure_result: Default::default(),
            inner_placeable: Rc::new(RefCell::new(InnerPlaceable::new())),
            outer_measurable_placeable: OuterMeasurePlaceable::new(),
            outer_layout_node: Rc::new(RefCell::new(LayoutNodeWrapperImpl::new())),
            layout_state: Default::default(),
        };

        let node = Rc::new(RefCell::new(node));
        {
            let mut node_mut = node.borrow_mut();
            node_mut.inner_placeable.borrow_mut().attach(Rc::downgrade(&node));
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

            let layout_node_wrapper: Rc<RefCell<dyn LayoutNodeWrapper>> = self.inner_placeable.clone();

        // let outer_wrapper = self.modifier.fold_out(layout_node_wrapper, &mut |modifier, to_wrap| {
        //     let mut wrapper = to_wrap;
        //
        //     wrapper
        // });
    }

    pub fn set_measure_policy(& self,
                              measure_policy: MultiChildrenMeasurePolicy) {
        self.inner_placeable.borrow_mut().measure_policy = measure_policy;
    }

    fn layout(width: usize, height: usize) -> MeasureResult {
        MeasureResult {
            width,
            height,
        }
    }

    pub(crate) fn adopt_child(& self, child: Rc<RefCell<LayoutNode>>) {
        self.inner_placeable.borrow_mut().adopt_child(child);
    }

    pub fn remeasure(&mut self, constraint: &Constraint) -> bool {
        let outer_placeable = &mut self.outer_measurable_placeable;
        outer_placeable.remeasure(constraint)
    }

    fn draw(canvas: &dyn Canvas) {}
}

impl Measurable for LayoutNode {
    fn measure(&mut self, constraint: &Constraint) -> &mut dyn Placeable {
        self.outer_measurable_placeable.measure(constraint)
    }
}