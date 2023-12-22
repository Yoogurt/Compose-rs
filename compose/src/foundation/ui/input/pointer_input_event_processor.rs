use crate::foundation::ui::input::pointer_event::PointerType;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use log::LevelFilter::Off;
use crate::foundation::geometry::Offset;
use crate::foundation::layout_node::LayoutNode;
use crate::foundation::ui::hit_test_result::HitTestResult;
use crate::foundation::ui::input::hit_path_tracker::HitPathTracker;
use crate::foundation::ui::input::internal_pointer_event::InternalPointerEvent;
use crate::foundation::ui::input::pointer_event::{PointerId, PointerInputChange};
use crate::foundation::ui::input::pointer_event_type::PointerInputEvent;
use crate::foundation::ui::input::process_result::ProcessResult;
use crate::foundation::utils::vec_extension::VecExtension;

pub(crate) trait PositionCalculator {
    fn screen_to_local(&self, position_on_screen: Offset<f32>) -> Offset<f32>;
    fn local_to_screen(&self, position_on_local: Offset<f32>) -> Offset<f32>;
}

pub(crate) struct PointerInputEventProcessor {
    root: Rc<RefCell<LayoutNode>>,
    is_processing: bool,
    pointer_input_change_event_producer: PointerInputChangeEventProducer,
    hit_result: HitTestResult,
    hit_path_tracker: HitPathTracker,
}

struct PointerInputData {
    uptime: u128,
    position_on_screen: Offset<f32>,
    down: bool,
    pointer_type: PointerType,
}

#[derive(Default)]
struct PointerInputChangeEventProducer {
    previous_pointer_input_data: HashMap<PointerId, PointerInputData>,
}

impl PointerInputChangeEventProducer {
    fn produce(&mut self, pointer_input_event: PointerInputEvent, position_calculator: &dyn PositionCalculator) -> InternalPointerEvent {
        let mut changes = HashMap::<PointerId, PointerInputChange>::new();

        pointer_input_event.pointers.iter().for_each(|pointer_input_event| {
            let previous_time: u128;
            let previous_position: Offset<f32>;
            let previous_down: bool;

            let previous_data = self.previous_pointer_input_data.get(&pointer_input_event.id);
            match previous_data {
                None => {
                    previous_time = pointer_input_event.uptime;
                    previous_position = pointer_input_event.position;
                    previous_down = false;
                }

                Some(previous_data) => {
                    previous_time = previous_data.uptime;
                    previous_down = previous_data.down;
                    previous_position = position_calculator.screen_to_local(previous_data.position_on_screen);
                }
            }

            changes.insert(pointer_input_event.id, PointerInputChange::new(
                pointer_input_event.id,
                pointer_input_event.uptime,
                pointer_input_event.position,
                pointer_input_event.down,
                pointer_input_event.pressure,
                previous_time,
                previous_position,
                previous_down,
                false,
                pointer_input_event.pointer_type,
                pointer_input_event.histroical.clone(),
                pointer_input_event.scroll_delta,
            ));

            if pointer_input_event.down {
                self.previous_pointer_input_data.insert(pointer_input_event.id, PointerInputData {
                    uptime: pointer_input_event.uptime,
                    position_on_screen: pointer_input_event.position_on_screen,
                    down: pointer_input_event.down,
                    pointer_type: pointer_input_event.pointer_type,
                });
            } else {
                self.previous_pointer_input_data.remove(&pointer_input_event.id);
            }
        });

        InternalPointerEvent::new(changes, pointer_input_event)
    }
}

impl PointerInputEventProcessor {
    pub(crate) fn new(root: Rc<RefCell<LayoutNode>>) -> PointerInputEventProcessor {
        let coordinates = root.borrow().get_coodinates();

        PointerInputEventProcessor {
            root,
            is_processing: false,
            pointer_input_change_event_producer: PointerInputChangeEventProducer::default(),
            hit_result: HitTestResult::new(),
            hit_path_tracker: HitPathTracker::new(coordinates),
        }
    }

    pub(crate) fn process(&mut self, event: PointerInputEvent, position_calculator: &dyn PositionCalculator, is_in_bounds: bool) -> ProcessResult {
        if self.is_processing {
            return ProcessResult::new(false, false);
        }

        self.is_processing = true;

        let internal_pointer_event = self.pointer_input_change_event_producer.produce(event, position_calculator);

        let is_hover = !internal_pointer_event.changes().values().any(|change| change.pressed || change.previous_pressed);

        internal_pointer_event.changes().values().for_each(|pointer_input_change| {
            if is_hover || pointer_input_change.changed_to_down_ignore_consumed() {
                let is_touch_event = pointer_input_change.pointer_type == PointerType::Touch;

                let hit_test_delegate = self.root.borrow().layout_node_hit_test_delegate.clone();
                hit_test_delegate.borrow().hit_test(pointer_input_change.position, &mut self.hit_result, is_touch_event, false);
                if self.hit_result.is_not_empty() {
                    self.hit_path_tracker.add_hit_path(pointer_input_change.id, self.hit_result.collect_node());
                    self.hit_result.clear();
                }
            }
        });
        self.is_processing = false;

        ProcessResult::new(false, false)
    }

    // pub(crate) fn process_pointer_input_event(&self, event: PointerInputEvent) {
    //     let hit_path_tracker = HitPathTracker::new(self.root.clone());
    //     let hit_path = hit_path_tracker.get_hit_path(event.location);
    //     let mut event = event;
    //     for layout_coordinates in hit_path {
    //         let layout_node = layout_coordinates.borrow().get_layout_node().unwrap();
    //         let layout_node_mut = layout_node.borrow_mut();
    //         let layout_node_layout_delegate = layout_node_mut.layout_node_layout_delegate.clone();
    //         let pointer_input_delegate = layout_node_layout_delegate.borrow().pointer_input_delegate.clone();
    //         drop(layout_node_mut);
    //         pointer_input_delegate.borrow_mut().process_pointer_input_event(&mut event);
    //     }
    // }
}