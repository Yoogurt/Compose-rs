use std::cell::RefCell;
use std::rc::Weak;

use crate::foundation::constraint::Constraints;
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::measurable::Measurable;
use crate::foundation::measure_result::MeasureResult;
use crate::foundation::measure_scope::MeasureScope;
use auto_delegate::Delegate;
use std::any::Any;
use std::fmt::{format, Debug, Formatter};

use super::node_coordinator::NodeCoordinator;
use super::placeable::Placeable;
use crate::foundation::geometry::IntOffset;
use crate::foundation::measurable::MultiChildrenMeasurePolicy;
use crate::foundation::node_coordinator_impl::NodeCoordinatorImpl;
use crate::foundation::placeable_place_at::PlaceablePlaceAt;
use crate::foundation::usage_by_parent::UsageByParent;
use std::ops::DerefMut;

#[derive(Delegate)]
pub(crate) struct InnerNodeCoordinator {
    #[to(
        Placeable,
        Measured,
        NodeCoordinatorTrait,
        MeasureScope,
        IntrinsicMeasurable
    )]
    pub(crate) node_coordinator_impl: NodeCoordinatorImpl,
    pub(crate) layout_node: Weak<RefCell<LayoutNode>>,
    pub(crate) measure_policy: MultiChildrenMeasurePolicy,
}

fn error_measure_policy(
    measure_scope: &mut dyn MeasureScope,
    _children: &mut [&mut dyn Measurable],
    _constraint: &Constraints,
) -> MeasureResult {
    panic!("no measure policy provided")
}

impl InnerNodeCoordinator {
    pub(crate) fn new() -> InnerNodeCoordinator {
        InnerNodeCoordinator {
            measure_policy: Box::new(error_measure_policy),
            layout_node: Weak::new(),
            node_coordinator_impl: NodeCoordinatorImpl::new(),
        }
    }

    pub(crate) fn attach(&mut self, layout_node: Weak<RefCell<LayoutNode>>) {
        self.layout_node = layout_node.clone();
        self.node_coordinator_impl.attach(layout_node);
    }

    pub(crate) fn set_measure_policy(&mut self, measure_policy: MultiChildrenMeasurePolicy) {
        self.measure_policy = measure_policy;
    }

    pub(crate) fn on_measured(&self) {
        println!("child {:p} measured {:?}\n", self, self.get_measured_size());
    }

    pub(crate) fn set_measured_result(&mut self, measure_result: MeasureResult) {
        dbg!(&measure_result);
        // self.set_measured_size(measure_result);
    }
}

impl Measurable for InnerNodeCoordinator {
    fn measure(&mut self, constraint: &Constraints) -> &mut dyn Placeable {
        { self.layout_node.upgrade().unwrap().borrow() }.for_each_child(|child| {
            child
                .borrow_mut()
                .get_measure_pass_delegate()
                .borrow_mut()
                .set_measured_by_parent(UsageByParent::NotUsed)
        });

        let measure_policy = &mut self.measure_policy;
        let measure_result = {
            let children_rc = self.layout_node.upgrade().unwrap().borrow().get_children();
            let children = children_rc.borrow_mut();

            let children_rc = children
                .iter()
                .map(|child| child.borrow_mut().layout_node_layout_delegate.clone())
                .collect::<Vec<_>>();
            let mut children_ref_mut = children_rc
                .iter()
                .map(|child| child.borrow_mut())
                .collect::<Vec<_>>();
            let mut children_ref_mut = children_ref_mut
                .iter_mut()
                .map(|child| child.deref_mut().as_measurable_mut())
                .collect::<Vec<_>>();
            let mut children_dyn_measurable = children_ref_mut
                .iter_mut()
                .map(|child| child.deref_mut())
                .collect::<Vec<_>>();

            let measure_scope = &mut self.node_coordinator_impl;
            measure_policy(measure_scope, &mut children_dyn_measurable[..], constraint)
        };
        self.set_measured_result(measure_result);

        self.on_measured();
        // self.handle_measured_result(measure_result);
        self
    }

    fn as_placeable_mut(&mut self) -> &mut dyn Placeable {
        self
    }

    fn as_measurable_mut(&mut self) -> &mut dyn Measurable {
        self
    }
}

impl PlaceablePlaceAt for InnerNodeCoordinator {
    fn place_at(&mut self, position: IntOffset, z_index: f32) {
        self.node_coordinator_impl.place_at(position, z_index)
    }
}

impl NodeCoordinator for InnerNodeCoordinator {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Debug for InnerNodeCoordinator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InnerNodeCoordinator")
            .field("node_coordinator_impl", &self.node_coordinator_impl)
            .field("layout_node", &self.layout_node)
            .field(
                "measure_policy",
                &format!("measure_policy: {:p}", &self.measure_policy),
            )
            .finish()
    }
}
