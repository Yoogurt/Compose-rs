use std::{rc::Rc, cell::RefCell};


use crate::foundation::composer::Composer;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;

use super::{constraint::Constraint, slot_table_type::SlotTableType, layout_node::LayoutNode, composer::ComposerInner, layout_node_guard::LayoutNodeGuard};

thread_local! {
    pub static COMPOSER : Composer = Composer::default()
}

impl ComposerInner {
    fn destroy(&mut self) {
        self.slot_table.data.clear();
        self.slot_table.index = 0;
        self.root = None;
        self.fix_up.clear();
        self.insert_up_fix_up.clear();
self.deferred_changes.clear();
    }


    fn dispatch_layout_to_first_layout_node(&self, _constraint: &Constraint) {
        for slot_table_type in &self.slot_table.data {
            match slot_table_type {
                SlotTableType::LayoutNodeType(_layout_node) => {
                    // let measure_result = layout_node.borrow_mut().measure(constraint);
                    // layout_node.borrow_mut().handle_measured_result(measure_result);
                    return;
                }
                _ => {}
            }
        }
    }

    fn attach_root_layout_node(&mut self, root: Rc<RefCell<LayoutNode>>) -> bool {
        if self.root.is_some() {
            return false;
        }

        self.root = Some(root);
        true
    }

    fn inserting(&self) -> bool {
        self.inserting
    }

    fn deattach_root_layout_node(&mut self) {
        self.root = None;
    }

    fn start_group(&mut self, hash: i64) {
        self.hash ^= hash;
        self.depth += 1;

        self.slot_table.push(SlotTableType::Group {
            hash: self.hash,
            depth: self.depth,
        });
    }

    fn end_group(&mut self, hash: i64) {
        self.depth -= 1;
        self.hash ^= hash;
    }

    fn start_node(&mut self) {

    }

    fn create_node(&mut self) -> Rc<RefCell<LayoutNode>> {
        let node = LayoutNode::new();
        let node = self.slot_table.push(SlotTableType::LayoutNodeType(node));

        match node {
            SlotTableType::LayoutNodeType(node) => {
                self.layout_node_stack.push(node.clone());
                return node.clone();
            }
            _ => {
                panic!("unexpect type")
            }
        }
    }

    fn record_fix_up(&mut self, fix_up: Box<dyn FnOnce()>) {
        self.fix_up.push(fix_up)
    }

    fn record_insert_up_fix_up(&mut self, fix_up: Box<dyn FnOnce()>) {
        self.insert_up_fix_up.push(fix_up)
    }

    fn record_deferred_change(&mut self, derred_change: Box<dyn FnOnce()>) {
        self.deferred_changes.push(derred_change)
    }

    fn apply_changes(&mut self) {
        let mut fix_up_insert = Vec::<Box<dyn FnOnce()>>::new();
        std::mem::swap(&mut self.insert_up_fix_up, &mut fix_up_insert);
        fix_up_insert.into_iter().for_each(|change| {
            change();
        });

        let mut fix_up = Vec::<Box<dyn FnOnce()>>::new();
        std::mem::swap(&mut self.fix_up, &mut fix_up);
        fix_up.into_iter().rev().for_each(|change| {
            change();
        });
    }

    fn apply_deferred_changes(&mut self) {
        let mut deferred_changes = Vec::<Box<dyn FnOnce()>>::new();
        std::mem::swap(&mut self.deferred_changes, &mut deferred_changes);
        deferred_changes.into_iter().for_each(|change| {
            change();
        });
    }

    fn end_node(&mut self) {
        let current = self.layout_node_stack.pop();

        match current {
            None => {
                panic!("unexpect current node no found")
            }

            Some(current) => {
                match self.layout_node_stack.last().cloned() {
                    Some(parent) => {
                        self.record_insert_up_fix_up(Box::new(move || {
                            LayoutNode::adopt_child(&parent, &current);
                        }));
                    }
                    None => {
                        // attach to root node
                        let root = self.root.clone().unwrap();
                        self.record_insert_up_fix_up(Box::new(move || {
                            LayoutNode::adopt_child(&root, &current);
                        }));
                    }
                }
            }
        }
    }
}

impl Composer {
    pub fn dispatch_layout_to_first_layout_node(constraint: &Constraint) {
        COMPOSER.with(|local_composer| {
            local_composer.inner.borrow().dispatch_layout_to_first_layout_node(constraint);
        })
    }

    pub(crate) fn attach_root_layout_node(root: Rc<RefCell<LayoutNode>>) -> bool {
        COMPOSER.with(|local_composer| {
            local_composer.inner.borrow_mut().attach_root_layout_node(root)
        })
    }

    pub fn destroy() {
        COMPOSER.with(|local_composer| {
            local_composer.inner.borrow_mut().destroy()
        })
    }

    pub(crate) fn detach_root_layout_node() {
        COMPOSER.with(|local_composer| {
            local_composer.inner.borrow_mut().deattach_root_layout_node();
        })
    }

    pub fn start_group(hash: i64) {
        COMPOSER.with(|local_composer| {
            local_composer.inner.borrow_mut().start_group(hash);
        })
    }

    pub(crate) fn start_node()  {
        COMPOSER.with(|local_composer| {
            local_composer.inner.borrow_mut().start_node()
        })
    }

    pub(crate) fn create_node() -> Rc<RefCell<LayoutNode>> {
        COMPOSER.with(|local_composer| {
            local_composer.inner.borrow_mut().create_node()
        })
    }

    pub(crate) fn record_fix_up(fix_up: Box<dyn FnOnce()>) {
        COMPOSER.with(move |local_composer| {
            local_composer.inner.borrow_mut().record_fix_up(fix_up)
        })
    }

    pub(crate) fn record_insert_up_fix_up(insert_up: Box<dyn FnOnce()>) {
        COMPOSER.with(move |local_composer| {
            local_composer.inner.borrow_mut().record_insert_up_fix_up(insert_up)
        })
    }

    pub(crate) fn record_deferred_change(&mut self, derred_change: Box<dyn FnOnce()>) {
        COMPOSER.with(move |local_composer| {
            local_composer.inner.borrow_mut().record_deferred_change(derred_change)
        })
    }

    pub fn apply_changes() {
        COMPOSER.with(move |local_composer| {
            local_composer.inner.borrow_mut().apply_changes()
        })
    }

    pub fn apply_deferred_changes() {
        COMPOSER.with(move |local_composer| {
            local_composer.inner.borrow_mut().apply_deferred_changes()
        })
    }

    pub(crate) fn end_node() {
        COMPOSER.with(|local_composer| {
            local_composer.inner.borrow_mut().end_node();
        })
    }

    pub(crate) fn inserting()-> bool{
        COMPOSER.with(|local_composer| {
            local_composer.inner.borrow().inserting()
        })
    }

    pub fn end_group(hash: i64) {
        COMPOSER.with(|local_composer| {
            local_composer.inner.borrow_mut().end_group(hash);
        })
    }

    pub fn validate_group(&self) -> bool {
        // return *self.depth.borrow() == 0 && *self.hash.borrow() == 0;
        return true;
    }

    pub fn skip_compose() {}

    pub fn skip_to_group() {}
}

impl Default for Composer {
    fn default() -> Self {
        Composer {
            inner: RefCell::new(Default::default()),
        }
    }
}