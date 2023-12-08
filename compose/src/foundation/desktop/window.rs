use minifb::{MouseButton, MouseMode};
use std::cell::RefCell;
use std::fmt::Display;
use crate::foundation::geometry::{Density, IntRect};
use crate::foundation::canvas::Canvas;
use std::time::Duration;
use crate as compose;
use minifb::{Key, KeyRepeat, Scale, ScaleMode, Window, WindowOptions};
use skia_safe::{AlphaType, ColorSpace, ColorType, ImageInfo, surfaces,
};
use std::rc::Rc;
use crate::foundation::bridge::skia_base_owner::SkiaBaseOwner;
use crate::foundation::composer::Composer;
use crate::foundation::drawing::canvas_impl::new_canvas;
use crate::foundation::geometry::IntSize;
use crate::foundation::measure_layout_defer_action_manager::MeasureLayoutDeferActionManager;
use crate::foundation::ui::compose_scene::ComposeScene;
use crate::foundation::ui::graphics::color::Color;
use crate::foundation::utils::result_extension::ResultExtension;

pub struct DesktopWindowOption {
    on_close_request: Option<Box<dyn Fn()>>,
    visible: bool,
    title: String,
    size: IntSize,
}

impl Default for DesktopWindowOption {
    fn default() -> Self {
        Self {
            on_close_request: None,
            visible: true,
            title: "Untitled".to_string(),
            size: IntSize::new(800, 500),
        }
    }
}

pub fn DesktopWindow(option: DesktopWindowOption,
                     content: impl Fn(),
                     diff: impl Fn()) {
    let window_width = option.size.width;
    let window_height = option.size.height;

    let mut windows = Window::new(
        &option.title,
        window_width,
        window_height,
        WindowOptions {
            scale: Scale::X1,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..Default::default()
        },
    ).unwrap();

    let mut buffer = vec![0; window_width * window_height];
    const BYTE_PER_PIXEL: usize = 4;

    let image_info = ImageInfo::new(
        (window_width as i32, window_height as i32),
        ColorType::BGRA8888,
        AlphaType::Opaque,
        Some(ColorSpace::new_srgb()),
    );

    let mut surface = unsafe {
        surfaces::wrap_pixels(
            &image_info,
            std::slice::from_raw_parts_mut(buffer.as_ptr() as *mut u8, 800 * 500 * BYTE_PER_PIXEL),
            window_width * BYTE_PER_PIXEL,
            None,
        )
    }.unwrap();

    let mut canvas = new_canvas(surface.canvas());
    let mut compose_view_rc = SkiaBaseOwner::new(IntRect::ZERO);
    let runtime = tokio::runtime::Builder::new_current_thread().build().unwrap();
    let mut redraw_need = Rc::new(RefCell::new(true));
    let redraw_capture = redraw_need.clone();
    let mut compose_scene = ComposeScene::new(runtime, Density::default(), Box::new(move || {
        redraw_capture.replace(true);
    }));
    compose_scene.attach(compose_view_rc.clone());

    let mut compose_view = compose_view_rc.borrow_mut();

    compose_view.set_content(content);
    drop(compose_view);

    Composer::apply_changes();
    Composer::apply_deferred_changes();

    // let mut compose_view = compose_view_rc.borrow_mut();
    // compose_view.no_insert_set_content(diff);
    //
    // drop(compose_view);
    // Composer::apply_changes();
    // Composer::apply_deferred_changes();

    Composer::debug_print();

    while windows.is_open() && !windows.is_key_pressed(Key::Escape, KeyRepeat::No) {
        {
            let mut compose_view = compose_view_rc.borrow_mut();
            let (width, height) = windows.get_size();
            compose_view.update_bound(IntRect::new(0, 0, width as i32, height as i32));

            MeasureLayoutDeferActionManager::with_manager(|defer_measure, defer_layout| {
                compose_view.dispatch_measure(width, height);
                defer_measure();
                compose_view.dispatch_layout();
                defer_layout();
            });

            if redraw_need.replace(false) {
                canvas.clear(Color::WHITE);
                compose_view.dispatch_draw(&mut canvas);
            }
        }

        windows.update_with_buffer(buffer.as_slice(), window_width, window_height).unwrap();
        process_mouse_event(&mut compose_scene, &windows);
        std::thread::sleep(Duration::from_millis(66));
    }

    compose_scene.detach(compose_view_rc.clone());
}

fn process_mouse_event(compose_scene: &mut ComposeScene, windows: &Window) {
    let Some(mouse_position) = windows.get_mouse_pos(MouseMode::Pass) else {
        return;
    };
    let left_mouse_button_pressed = windows.get_mouse_down(MouseButton::Left);
    let right_mouse_button_pressed = windows.get_mouse_down(MouseButton::Right);

    let event_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
    // dbg!(mouse_position);
    compose_scene.on_mouse_event(mouse_position.0, mouse_position.1, event_time, left_mouse_button_pressed, right_mouse_button_pressed)
}