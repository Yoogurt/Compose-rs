use std::cell::{RefCell, RefMut};
use std::rc::Weak;
use std::rc::Rc;

use crate::foundation::constraint::Constraints;
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::measurable::Measurable;
use crate::foundation::measure_result::{MeasureResult, MeasureResultProvider};
use crate::foundation::measure_scope::MeasureScope;
use auto_delegate::Delegate;
use std::any::Any;
use std::fmt::{format, Debug, Formatter};

use super::node_coordinator::NodeCoordinator;
use super::placeable::Placeable;
use crate::foundation::geometry::{IntOffset, IntSize};
use crate::foundation::measurable::MultiChildrenMeasurePolicy;
use crate::foundation::node_coordinator_impl::NodeCoordinatorImpl;
use crate::foundation::placeable_place_at::PlaceablePlaceAt;
use crate::foundation::usage_by_parent::UsageByParent;
use std::ops::{Deref, DerefMut};
use compose_foundation_macro::AnyConverter;
use crate::foundation::canvas::Canvas;
use crate::foundation::layout_node_layout_delegate::LayoutNodeLayoutDelegate;
use crate::foundation::measure_pass_delegate::MeasurePassDelegate;
use crate::foundation::node_coordinator::PerformDrawTrait;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;
use crate::foundation::utils::self_reference::SelfReference;

#[derive(Delegate, AnyConverter)]
pub(crate) struct InnerNodeCoordinator {
    #[to(
    Placeable,
    Measured,
    NodeCoordinator,
    NodeCoordinatorTrait,
    MeasureScope,
    IntrinsicMeasurable,
    LookaheadCapablePlaceable,
    TailModifierNodeProvider,
    MeasureResultProvider
    )]
    pub(crate) node_coordinator_impl: NodeCoordinatorImpl,
    pub(crate) layout_node: Weak<RefCell<LayoutNode>>,
    pub(crate) measure_policy: MultiChildrenMeasurePolicy,
    pub(crate) measure_pass_delegate: Weak<RefCell<MeasurePassDelegate>>,

    weak_this: Weak<RefCell<Self>>,
}

impl DerefMut for InnerNodeCoordinator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node_coordinator_impl
    }
}

impl Deref for InnerNodeCoordinator {
    type Target = dyn NodeCoordinator;

    fn deref(&self) -> &Self::Target {
        &self.node_coordinator_impl
    }
}

fn error_measure_policy(
    _measure_scope: &dyn MeasureScope,
    _children: &mut [&mut dyn Measurable],
    _constraint: &Constraints,
) -> MeasureResult {
    panic!("no measure policy provided")
}

impl InnerNodeCoordinator {
    pub(crate) fn new() -> Rc<RefCell<InnerNodeCoordinator>> {
        let mut result = InnerNodeCoordinator {
            measure_policy: Box::new(error_measure_policy),
            layout_node: Weak::new(),
            node_coordinator_impl: NodeCoordinatorImpl::new(),
            measure_pass_delegate: Weak::new(),
            weak_this: Weak::new(),
        }.wrap_with_rc_refcell();

        let this: Rc<RefCell<dyn PerformDrawTrait>> = result.clone();

        {
            let mut result_mut = result.borrow_mut();
            result_mut.node_coordinator_impl.attach_vtable(Rc::downgrade(&this));

            result_mut.weak_this = Rc::downgrade(&result);
        }
        result
    }

    pub(crate) fn attach(&mut self, layout_node: &Rc<RefCell<LayoutNode>>, measure_pass_delegate: &Rc<RefCell<MeasurePassDelegate>>) {
        self.layout_node = Rc::downgrade(layout_node);
        self.measure_pass_delegate = Rc::downgrade(measure_pass_delegate);
        self.node_coordinator_impl.attach(layout_node);
    }

    pub(crate) fn set_measure_policy(&mut self, measure_policy: MultiChildrenMeasurePolicy) {
        self.measure_policy = measure_policy;
    }

    pub(crate) fn on_measured(&self) {
        println!("child {:p} measured {:?}\n", self, self.get_measured_size());
    }
}

impl Measurable for InnerNodeCoordinator {
    fn measure(&mut self, constraint: &Constraints) -> (IntSize, Rc<RefCell<dyn Placeable>>) {
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

            let measure_scope = &self.node_coordinator_impl;
            measure_policy(measure_scope, &mut measurable_mut[..], constraint)
        };

        let size: IntSize = measure_result.as_int_size();
        self.set_measured_result(Some(measure_result));
        self.on_measured();

        (size, self.as_placeable())
    }

    fn as_placeable(&mut self) -> Rc<RefCell<dyn Placeable>> {
        self.get_self().upgrade().unwrap()
    }

    fn as_measurable_mut(&mut self) -> &mut dyn Measurable {
        self
    }
}

impl SelfReference for InnerNodeCoordinator {
    fn get_self(&self) -> Weak<RefCell<Self>> {
        self.weak_this.clone()
    }
}

impl PlaceablePlaceAt for InnerNodeCoordinator {
    fn place_at(&mut self, position: IntOffset, z_index: f32) {
        self.node_coordinator_impl.place_at(position, z_index);
        self.on_placed();
    }
}

impl PerformDrawTrait for InnerNodeCoordinator {
    fn perform_draw(&self, canvas: &mut dyn Canvas) {
        let children = self.layout_node.upgrade().unwrap().borrow().z_sort_children();

        children.iter().for_each(|child| {
            let measure_pass_delegate = child.borrow().get_measure_pass_delegate();
            if measure_pass_delegate.borrow().is_placed {
                child.borrow().layout_node_draw_delegate.borrow_mut().draw(canvas);
            }
        });
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
