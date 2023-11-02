use std::any::Any;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};

use std::ops::DerefMut;
use std::rc::Weak;
use crate::foundation::layout_node::UsageByParent;
use crate::foundation::measurable::MultiChildrenMeasurePolicy;

use super::constraint::Constraint;
use super::inner_node_coordinator::InnerNodeCoordinator;
use super::layout_node::LayoutNode;
use super::layout_receiver::MeasureScope;
use super::placeable::{Placeable, PlaceablePlaceAt};
use super::node_coordinator::{NodeCoordinatorImpl, NodeCoordinator};
use super::measurable::Measurable;
use super::measure_result::MeasureResult;

fn error_measure_policy(measure_scope: &mut dyn MeasureScope, _children: &mut [&mut dyn Measurable], _constraint: &Constraint) -> MeasureResult {
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
    fn measure(&mut self, constraint: &Constraint) -> &mut dyn Placeable {
        { self.layout_node.upgrade().unwrap().borrow() }.for_each_child(|child| {
            child.borrow_mut().get_measure_pass_delegate().borrow_mut().set_measured_by_parent(UsageByParent::NotUsed)
        });

        let measure_policy = &mut self.measure_policy;
        let measure_result = {
            let children_rc = self.layout_node.upgrade().unwrap().borrow().get_children();
            let children = children_rc.borrow_mut();

            let children_rc = children.iter().map(|child| {
                child.borrow_mut().layout_node_layout_delegate.clone()
            }).collect::<Vec<_>>();
            let mut children_ref_mut = children_rc.iter().map(|child| {
                child.borrow_mut()
            }).collect::<Vec<_>>();
            let mut children_ref_mut = children_ref_mut.iter_mut().map(|child| {
                child.deref_mut().as_measurable_mut()
            }).collect::<Vec<_>>();
            let mut children_dyn_measurable = children_ref_mut.iter_mut().map(|child| {
                child.deref_mut()
            }).collect::<Vec<_>>();

            let measure_scope = &mut self.node_coordinator_impl;
            measure_policy(measure_scope, &mut children_dyn_measurable[..], constraint)
        };
        self.set_measured_result(measure_result);

        self.on_measured();
        // self.handle_measured_result(measure_result);
        &mut self.node_coordinator_impl
    }
}

impl PlaceablePlaceAt for InnerNodeCoordinator {
    fn place_at(&mut self,position:super::geometry::IntOffset,z_index:f32) {
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
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}