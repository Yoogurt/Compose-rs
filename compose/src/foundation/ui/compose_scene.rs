use crate::foundation::ui::input::internal_pointer_input::PointerInputEventData;
use std::marker::PhantomData;
use tokio::runtime::Runtime;
use crate::foundation::geometry::{Density, Offset};
use std::any::Any;
use std::cell::RefCell;
use crate::foundation::bridge::skia_base_owner::SkiaBaseOwner;
use crate::foundation::node::GesstureOwner;
use crate::foundation::ui::input::pointer_button::{PointerButton, PointerButtons};
use crate::foundation::ui::input::pointer_event::{PointerEventType, PointerType};
use crate::foundation::ui::input::pointer_event_type::PointerInputEvent;
use std::rc::Rc;
use crate::foundation::utils::option_extension::OptionThen;

pub struct ComposeScene {
    coroutine_scope: Runtime,
    density: Density,
    invalidate: Box<dyn FnMut() + 'static>,

    default_pointer_state_tracker: DefaultPointerStateTracker,

    last_left_pressed: bool,
    last_right_pressed: bool,

    owners: Vec<Rc<RefCell<SkiaBaseOwner>>>,
    gesture_owner: Option<Rc<RefCell<SkiaBaseOwner>>>,

    last_hover_owner: Option<Rc<RefCell<SkiaBaseOwner>>>,
}

#[derive(Default)]
struct DefaultPointerStateTracker {
    pub(crate) buttons: PointerButtons,
}

impl DefaultPointerStateTracker {
    pub fn new() -> Self {
        Self {
            buttons: PointerButtons::default()
        }
    }

    fn on_pointer_event(&mut self, button: Option<PointerButton>, event_type: PointerEventType) {
        match event_type {
            PointerEventType::Press => { self.buttons = self.buttons.copy_for(button.unwrap_or(PointerButton::Primary), true) }
            PointerEventType::Release => { self.buttons = self.buttons.copy_for(button.unwrap_or(PointerButton::Primary), false) }
            _ => {}
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Pointer {
    id: u64,
    position: Offset<f32>,
    pressed: bool,
    pointer_type: PointerType,
    pressure: f32,
}

impl Default for Pointer {
    fn default() -> Self {
        Self {
            id: 0,
            position: Offset::zero(),
            pressed: false,
            pointer_type: PointerType::Mouse,
            pressure: 1.0,
        }
    }
}

impl ComposeScene {
    pub fn new(runtime: Runtime, density: Density, invalidate: Box<dyn FnMut() + 'static>) -> Self {
        Self {
            coroutine_scope: runtime,
            density,
            invalidate,
            default_pointer_state_tracker: DefaultPointerStateTracker::new(),

            last_left_pressed: false,
            last_right_pressed: false,

            owners: vec![],
            gesture_owner: None,
            last_hover_owner: None,
        }
    }

    pub fn attach(&mut self, owner: Rc<RefCell<SkiaBaseOwner>>) {
        {
            let owner = owner.borrow_mut();
        }

        self.owners.push(owner);
    }

    pub fn detach(&mut self, owner: Rc<RefCell<SkiaBaseOwner>>) {
        self.owners.retain(|o| !Rc::ptr_eq(o, &owner));
    }

    pub fn on_mouse_event(&mut self, x: f32, y: f32, time_millis: u128, left_mouse_button_pressed: bool, right_mouse_button_pressed: bool) {
        let trigger_left_pressed = !self.last_left_pressed && left_mouse_button_pressed;
        let trigger_right_pressed = !self.last_right_pressed && right_mouse_button_pressed;
        let trigger_left_release = self.last_left_pressed && !left_mouse_button_pressed;
        let trigger_right_release = self.last_right_pressed && !right_mouse_button_pressed;

        self.last_left_pressed = left_mouse_button_pressed;
        self.last_right_pressed = right_mouse_button_pressed;

        if trigger_left_pressed {
            self.send_pointer_event(
                PointerEventType::Press,
                Offset::new(x, y),
                None,
                time_millis,
                PointerType::Mouse,
                Some(PointerButton::Primary),
                Offset::zero(),
            );
            return;
        }

        if trigger_right_pressed {
            self.send_pointer_event(
                PointerEventType::Press,
                Offset::new(x, y),
                None,
                time_millis,
                PointerType::Mouse,
                Some(PointerButton::Secondary),
                Offset::zero(),
            );
            return;
        }

        if trigger_left_release {
            self.send_pointer_event(
                PointerEventType::Release,
                Offset::new(x, y),
                None,
                time_millis,
                PointerType::Mouse,
                Some(PointerButton::Primary),
                Offset::zero(),
            );
        }

        if trigger_right_release {
            self.send_pointer_event(
                PointerEventType::Release,
                Offset::new(x, y),
                None,
                time_millis,
                PointerType::Mouse,
                Some(PointerButton::Secondary),
                Offset::zero(),
            );
        }

        self.send_pointer_event(
            PointerEventType::Move,
            Offset::new(x, y),
            None,
            time_millis,
            PointerType::Mouse,
            None,
            Offset::zero(),
        );
    }

    fn send_pointer_event(&mut self,
                          event_type: PointerEventType,
                          position: Offset<f32>,
                          buttons: Option<PointerButtons>,
                          time_millis: u128,
                          pointer_type: PointerType,
                          button: Option<PointerButton>,
                          scroll_delta: Offset<f32>) {
        self.default_pointer_state_tracker.on_pointer_event(button.clone(), event_type);

        let actual_buttons = buttons.clone().unwrap_or(self.default_pointer_state_tracker.buttons);

        let pointers = vec![Pointer {
            id: 0,
            position,
            pressed: actual_buttons.are_any_pressed(),
            pointer_type,
            ..Default::default()
        }];

        let buttons = actual_buttons;
        let event = PointerInputEvent::new(
            event_type,
            time_millis,
            pointers.into_iter().map(|pointer| {
                PointerInputEventData {
                    id: pointer.id,
                    uptime: time_millis,
                    position: pointer.position,
                    position_on_screen: pointer.position,
                    down: pointer.pressed,
                    pointer_type: pointer.pointer_type,
                    pressure: pointer.pressure,
                    scroll_delta,
                }
            }).collect(),
            buttons,
            button,
            scroll_delta,
        );

        self.process_pointer_event(event);
    }

    fn process_press(&mut self, event: PointerInputEvent) {
        if let Some(gesture_owner) = self.gesture_owner.as_ref() {
            gesture_owner.borrow_mut().process_pointer_input(event);
            return;
        }

        let position = event.pointers.first().unwrap().position.as_int_offset();
        self.owners.iter().rev().any(|owner| {
            if owner.borrow().is_in_bound(position) {
                owner.borrow_mut().process_pointer_input(event.clone());
                self.gesture_owner = Some(owner.clone());
                return true;
            }

            false
        });
    }

    fn hoverd_owner(&self, event: PointerInputEvent) -> Option<Rc<RefCell<SkiaBaseOwner>>> {
        let position = event.pointers.first().unwrap().position.as_int_offset();
        self.owners.iter().rev().find(|owner| owner.borrow().is_in_bound(position)).cloned()
    }

    fn process_hover(&mut self, event: PointerInputEvent, owner: Option<Rc<RefCell<SkiaBaseOwner>>>) -> bool {
        if event.pointers.iter().any(|event| event.pointer_type != PointerType::Mouse) {
            return false;
        }

        match (owner.as_ref(), self.last_hover_owner.as_ref()) {
            (Some(owner), Some(last_hover_owner)) => {
                if Rc::ptr_eq(owner, &last_hover_owner) {
                    return false;
                }
            }
            _ => {}
        }

        self.last_hover_owner.as_ref().then(|last_hover_owner| {
            let mut event = event.clone();
            event.event_type = PointerEventType::Exit;
            last_hover_owner.borrow_mut().process_pointer_input(event);
        });

        owner.as_ref().then(|owner| {
            let mut event = event.clone();
            event.event_type = PointerEventType::Enter;
            owner.borrow_mut().process_pointer_input(event);
        });

        self.last_hover_owner = owner;
        true
    }

    fn process_move(&mut self, event: PointerInputEvent) {
        let owner = if event.is_gesture_in_progress() {
            self.gesture_owner.clone()
        } else if event.event_type == PointerEventType::Exit {
            None
        } else {
            self.hoverd_owner(event.clone())
        };

        if self.process_hover(event.clone(), owner.clone()) {
            return;
        }

        owner.then(|owner| owner.borrow_mut().process_pointer_input(event));
    }

    fn process_pointer_event(&mut self, event: PointerInputEvent) {
        match event.event_type {
            PointerEventType::Press => {
                self.process_press(event);
            }
            PointerEventType::Release => {}
            PointerEventType::Move|PointerEventType::Enter|PointerEventType::Exit => {
                self.process_move(event);
            }
            PointerEventType::Scroll => {}
            _ => {}
        }
    }
}