use std::marker::PhantomData;
use tokio::runtime::Runtime;
use crate::foundation::geometry::{Density, Offset};
use std::any::Any;
use crate::foundation::ui::input::pointer_button::PointerButton;
use crate::foundation::ui::input::pointer_event::PointerEventType;

pub struct ComposeScene {
    coroutine_scope: Runtime,
    density: Density,
    invalidate: Box<dyn FnMut() + 'static>,
}

impl ComposeScene {
    pub fn new(runtime: Runtime, density: Density, invalidate: Box<dyn FnMut() + 'static>) -> Self {
        Self {
            coroutine_scope: runtime,
            density,
            invalidate,
        }
    }

    pub fn on_mouse_event(&mut self, x: f32, y: f32, time_millis: u64, ) {

    }

    fn send_pointer_event(&mut self, event_type: PointerEventType, position: Offset<f32>, time_millis: u64, button: Option<PointerButton>) {

    }
}