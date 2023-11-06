use std::cell::{RefCell, RefMut};
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
use crate::foundation::canvas::Canvas;
use crate::foundation::layout_node_layout_delegate::LayoutNodeLayoutDelegate;
use crate::foundation::node_coordinator::PerformDrawTrait;
use crate::foundation::oop::AnyConverter;
use crate::implement_any_by_self;

#[derive(Delegate)]
pub(crate) struct InnerNodeCoordinator {
    #[to(
    Placeable,
    Measured,
    NodeCoordinator,
    NodeCoordinatorTrait,
    MeasureScope,
    IntrinsicMeasurable,
    LookaheadCapablePlaceable,
    TailModifierNodeProvider
    )]
    pub(crate) node_coordinator_impl: NodeCoordinatorImpl,
    pub(crate) layout_node: Weak<RefCell<LayoutNode>>,
    pub(crate) measure_policy: MultiChildrenMeasurePolicy,
}

fn error_measure_policy(
    _measure_scope: &mut dyn MeasureScope,
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

            let layout_node_layout_delegate_rc = children
                .iter()
                .map(|child| child.borrow_mut().layout_node_layout_delegate.clone())
                .collect::<Vec<_>>();
            let mut layout_node_layout_delegate_ref_mut: Vec<RefMut<LayoutNodeLayoutDelegate>> = layout_node_layout_delegate_rc
                .iter()
                .map(|child| child.borrow_mut())
                .collect::<Vec<_>>();
            let mut measurable_ref_mut: Vec<RefMut<dyn Measurable>> = layout_node_layout_delegate_ref_mut
                .iter_mut()
                .map(|child| child.deref_mut().as_measurable_mut())
                .collect::<Vec<_>>();
            let mut measurable_mut: Vec<&mut dyn Measurable> = measurable_ref_mut
                .iter_mut()
                .map(|child| { child.deref_mut() })
                .collect::<Vec<_>>();

            let measure_scope = &mut self.node_coordinator_impl;
            measure_policy(measure_scope, &mut measurable_mut[..], constraint)
        };
        self.set_measured_result(measure_result);

        self.on_measured();
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

implement_any_by_self!(InnerNodeCoordinator);
impl PerformDrawTrait for InnerNodeCoordinator {
    fn perform_draw(&mut self, canvas: &mut dyn Canvas) {
        panic!("performing drawing")
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
