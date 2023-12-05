use std::cell::{RefCell, RefMut};
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::rc::Weak;

use auto_delegate::Delegate;
use compose_foundation_macro::AnyConverter;

use crate::foundation::canvas::Canvas;
use crate::foundation::composer::Composer;
use crate::foundation::constraint::Constraints;
use crate::foundation::geometry::{IntOffset, IntSize};
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::layout_node_layout_delegate::LayoutNodeLayoutDelegate;
use crate::foundation::measurable::{Measurable, MultiChildrenMeasurePolicyDelegate};
use crate::foundation::measurable::MultiChildrenMeasurePolicy;
use crate::foundation::measure_layout_defer_action_manager::MeasureLayoutDeferActionManager;
use crate::foundation::measure_pass_delegate::MeasurePassDelegate;
use crate::foundation::measure_result::{MeasureResult, MeasureResultProvider};
use crate::foundation::measure_scope::MeasureScope;
use crate::foundation::node_chain::NodeChain;
use crate::foundation::node_coordinator::{PerformDrawTrait, PerformMeasureHelper};
use crate::foundation::node_coordinator_impl::NodeCoordinatorImpl;
use crate::foundation::placeable_place_at::PlaceablePlaceAt;
use crate::foundation::usage_by_parent::UsageByParent;
use crate::foundation::utils::option_extension::OptionThen;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;
use crate::foundation::utils::self_reference::SelfReference;

use super::node_coordinator::NodeCoordinator;
use super::placeable::Placeable;

#[derive(Delegate, AnyConverter)]
pub(crate) struct InnerNodeCoordinator {
    #[to(
    Placeable,
    Measured,
    NodeCoordinator,
    DrawableNodeCoordinator,
    NodeCoordinatorTrait,
    MeasureScope,
    IntrinsicMeasurable,
    LookaheadCapablePlaceable,
    TailModifierNodeProvider,
    MeasureResultProvider,
    ParentDataGenerator,
    LayoutCoordinates
    )]
    pub(crate) node_coordinator_impl: NodeCoordinatorImpl,
    pub(crate) layout_node: Weak<RefCell<LayoutNode>>,
    pub(crate) measure_policy: MultiChildrenMeasurePolicy,
    pub(crate) measure_pass_delegate: Weak<RefCell<MeasurePassDelegate>>,

    weak_this: Weak<RefCell<Self>>,
    identify: u32,
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

fn error_measure_policy() -> MultiChildrenMeasurePolicy {
    MultiChildrenMeasurePolicyDelegate(|_, _, _| {
        panic!("no measure policy provided")
    })
}

impl InnerNodeCoordinator {
    pub(crate) fn new() -> Rc<RefCell<InnerNodeCoordinator>> {
        let mut result = InnerNodeCoordinator {
            measure_policy: error_measure_policy(),
            layout_node: Weak::new(),
            node_coordinator_impl: NodeCoordinatorImpl::new(),
            measure_pass_delegate: Weak::new(),
            weak_this: Weak::new(),
            identify: 0,
        }.wrap_with_rc_refcell();

        {
            let mut result_mut = result.borrow_mut();
            result_mut.weak_this = Rc::downgrade(&result);

            result_mut.node_coordinator_impl.set_vtable_perform_draw_trait(Rc::downgrade(&(result.clone() as Rc<RefCell<dyn PerformDrawTrait>>)));
        }
        result
    }

    pub(crate) fn attach(&mut self, identify: u32,
                         layout_node: &Rc<RefCell<LayoutNode>>,
                         measure_pass_delegate: &Rc<RefCell<MeasurePassDelegate>>,
                         node_chain: &Rc<RefCell<NodeChain>>) {
        self.identify = identify;
        self.layout_node = Rc::downgrade(layout_node);
        self.measure_pass_delegate = Rc::downgrade(measure_pass_delegate);
        self.node_coordinator_impl.attach(layout_node, node_chain);
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
        self.perform_measure(constraint, move |this| {
            { this.layout_node.upgrade().unwrap().borrow() }.for_each_child(|child| {
                child
                    .borrow_mut()
                    .get_measure_pass_delegate()
                    .borrow_mut()
                    .set_measured_by_parent(UsageByParent::NotUsed)
            });

            let measure_result = {
                let mut measure_policy = this.measure_policy.borrow_mut();

                let children_rc = this.layout_node.upgrade().unwrap().borrow().get_children();
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

                let measure_scope = &this.node_coordinator_impl;
                measure_policy(measure_scope, &mut measurable_mut[..], constraint)
            };

            let size: IntSize = measure_result.as_int_size();
            this.set_measured_result(measure_result);
            this.on_measured();

            (size, this.as_placeable())
        })
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

        let this = self.get_self();
        MeasureLayoutDeferActionManager::record_layout(move || {
            this.upgrade().then(|this| {
                this.borrow().on_placed();
            })
        });
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