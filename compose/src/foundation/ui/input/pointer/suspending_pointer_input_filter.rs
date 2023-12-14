use std::fmt::{Debug, Formatter};
use std::process::Output;
use crate::foundation::modifier::{Modifier, ModifierNodeElement, ModifierNodeImpl};
use crate::foundation::ui::input::pointer_event::{PointerEvent, PointerEventPass, PointerEventType};
use std::future::Future;
use auto_delegate::Delegate;
use compose_foundation_macro::ModifierElement;
use crate::foundation::geometry::IntSize;
use crate::foundation::modifier_node::PointerInputModifierNode;
use std::rc::Rc;

pub struct PointerInputScope {
    // fn get_size(&self) -> IntSize;
}

pub struct AwaitPointerEventScope;

impl AwaitPointerEventScope {
    pub async fn await_pointer_event(&self, pass: PointerEventPass) -> PointerEvent {
        todo!()
    }
}

impl PointerInputScope {
    pub fn await_pointer_event_scope<F>(&self, block: impl Fn(AwaitPointerEventScope) -> F) where F: Future<Output=()> {}
}

impl Modifier {
    pub fn pointer_input(self, block: impl Fn(PointerInputScope) + 'static) -> Modifier {
        self.then(suspend_pointer_input_element(Rc::new(block)))
    }
}

fn suspend_pointer_input_element(block: Rc<dyn Fn(PointerInputScope) + 'static>) -> Modifier {
    let block_for_update = block.clone();

    ModifierNodeElement(
        "SuspendPointerInputElement",
        move || SuspendPointerInputElement {
            pointer_input_handler: block.clone(),
            node_impl: Default::default(),
        }, move |modifier| {
            modifier.pointer_input_handler = block_for_update.clone();
        })
}

#[derive(Delegate, ModifierElement)]
#[Impl(PointerInput)]
struct SuspendPointerInputElement {
    pointer_input_handler: Rc<dyn Fn(PointerInputScope) + 'static>,

    #[to(ModifierNode)]
    node_impl: ModifierNodeImpl,
}

impl PointerInputModifierNode for SuspendPointerInputElement {}

impl Debug for SuspendPointerInputElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SuspendPointerInputElement")
            .field("node_impl", &self.node_impl)
            .finish()
    }
}