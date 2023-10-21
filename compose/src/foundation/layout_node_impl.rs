use std::cell::RefCell;
use std::ops::{DerefMut};
use std::rc::Rc;
use super::{Measurable, Canvas, Placeable, LayoutNode, MeasurePolicyDelegate, Modifier, Constraint, MeasureResult, InnerPlaceable, OuterPlaceable, LayoutNodeWrapper, LayoutState, LayoutReceiver};

impl Measurable for LayoutNode {
    fn measure(&mut self, constraint: &Constraint) -> Placeable {
        Placeable {
            width: 0,
            height: 0,
            measured_width: 0,
            measured_height: 0,
            left: 0,
            top: 0,
        }
    }
}

impl LayoutNode {
    pub(crate) fn new() -> Self {
        let node = LayoutNode {
            children: Default::default(),
            modifier: Default::default(),
            measure_policy: None,
            parent_data: None,
            measure_result: Default::default(),
            inner_placeable: InnerPlaceable::new(),
            inner_layout_node: LayoutNodeWrapper::new(),
            outer_placeable: OuterPlaceable::new(),
            outer_layout_node: LayoutNodeWrapper::new(),
            layout_state: Default::default(),
        };

        node
    }

    pub fn update(&mut self,
                  modifier: Modifier,
                  measure_policy: MeasurePolicyDelegate) {
        self.modifier = modifier;
        self.measure_policy = Some(measure_policy);
    }

    fn layout(width: usize, height: usize) -> MeasureResult {
        MeasureResult {
            width,
            height,
        }
    }

    pub fn handle_measured_result(&mut self, measure_result: MeasureResult) {
        dbg!(&measure_result);
        self.inner_placeable.measure_result = measure_result;
    }

    pub(crate) fn adopt_child(&mut self, child: Rc<RefCell<LayoutNode>>) {
        self.children.push(child);
    }

    pub fn measure(&self, constraint: &Constraint) -> MeasureResult {
        let mut children_ref = self.children.iter().map(|child| {
            child.borrow_mut()
        }).collect::<Vec<_>>();

        let mut children = children_ref.iter_mut().map(|value| {
            value.deref_mut() as &mut dyn Measurable
        }).collect::<Vec<_>>();

        match self.measure_policy {
            Some(measure_policy_delegate) => {
                let layout_receiver = LayoutReceiver::new();
                return measure_policy_delegate(layout_receiver, &mut children[..], constraint);
            }
            _ => {}
        }

        Self::layout(0, 0)
    }

    pub fn remeasure(&mut self, constraint: &Constraint) -> bool {
        let outer_placeable = &mut self.outer_placeable;
        outer_placeable.remeasure(self.layout_state, constraint, |layout_state| {
            self.layout_state = layout_state;
        })
    }

    fn draw(canvas: &dyn Canvas) {}
}
