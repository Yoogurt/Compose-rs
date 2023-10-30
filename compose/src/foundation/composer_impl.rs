
use std::{rc::Rc, cell::RefCell};


use crate::foundation::composer::Composer;

use super::{constraint::Constraint, slot_table_type::SlotTableType, layout_node::LayoutNode, composer::ComposerInner, layout_node_guard::LayoutNodeGuard};

thread_local! {
    pub static COMPOSER : Composer = Composer::default()
}

impl ComposerInner {
    pub fn destroy(&mut self) {
        self.slot_table.data.clear();
        self.slot_table.index = 0;
        self.root = None;
    }

    pub fn dispatch_layout_to_first_layout_node(&self, _constraint: &Constraint) {
        for slot_table_type in &self.slot_table.data {
            match slot_table_type {
                SlotTableType::LayoutNodeType(_layout_node) => {
                    // let measure_result = layout_node.borrow_mut().measure(constraint);
                    // layout_node.borrow_mut().handle_measured_result(measure_result);
                    return
                }
                _=> {}
            }
        }
    }

    pub fn attach_root_layout_node(&mut self, root : Rc<RefCell<LayoutNode>>) -> bool {
        if self.root.is_some() {
            return false;
        }

        self.root = Some(root);
        true
    }

    pub fn deattach_root_layout_node(&mut self) {
        self.root = None;
    }

    pub fn begin_group(&mut self, hash: i64) {
        self.hash ^= hash;
        self.depth += 1;

        self.slot_table.push(SlotTableType::Group {
            hash: self.hash,
            depth: self.depth
        });
    }

    pub fn end_group(&mut self, hash: i64) {
        self.depth -= 1;
        self.hash ^= hash;
    }

    pub fn begin_node(&mut self) -> Rc<RefCell<LayoutNode>> {
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

    pub fn end_node(&mut self) {
        let current = self.layout_node_stack.pop();

        match current {
            None => {
                panic!("unexpect current node no found")
            }

            Some(current) => {
                match self.layout_node_stack.last() {
                    Some(parent) => {
                        parent.borrow_mut().adopt_child(current);
                    }
                    None => {
                        // attach to root node
                        // self.root.clone().unwrap().borrow_mut().adopt_child(current);
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

    pub(crate) fn attach_root_layout_node( root : Rc<RefCell<LayoutNode>>) -> bool {
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

    pub fn begin_group(hash: i64) {
        COMPOSER.with(|local_composer| {
            local_composer.inner.borrow_mut().begin_group(hash);
        })
    }

    pub fn begin_node() -> LayoutNodeGuard {
        COMPOSER.with(|local_composer| {
            LayoutNodeGuard::new(local_composer.inner.borrow_mut().begin_node())
        })
    }

    pub fn end_node(_guard: LayoutNodeGuard) {
        COMPOSER.with(|local_composer| {
            local_composer.inner.borrow_mut().end_node();
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

impl Default for ComposerInner {
    fn default() -> Self {
        ComposerInner {
            hash: 0,
            depth: 0,
            insertion: false,
            layout_node_stack: Default::default(),
            slot_table: Default::default(),
            root: None,
        }
    }
}