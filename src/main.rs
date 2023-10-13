#![allow(warnings)]

use std::hash::Hash;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use skia_safe::{
    surface, surfaces, AlphaType, Color, Color4f, ColorSpace, ColorType, ImageInfo, Rect, Surface,
};
use std::time::Duration;
use compose::foundation::{Composer};
use compose_macro::Compose;

#[Compose]
fn test(param1: i32, param2: u64, param3: Box<u64>) {
    let mut current_composer : &Composer = current_composer;
    println!("hello world");
}


// fn run_skia() {
//     // test(1, 2);
//     let mut window = Window::new(
//         "Compose",
//         800,
//         500,
//         WindowOptions {
//             scale: Scale::X1,
//             scale_mode: ScaleMode::AspectRatioStretch,
//             ..Default::default()
//         },
//     )
//         .unwrap();
//
//     let mut buffer = [0u32; 800 * 500];
//     const BYTE_PER_PIXEL: usize = 4;
//
//     let image_info = ImageInfo::new(
//         (800, 500),
//         ColorType::BGRA8888,
//         AlphaType::Opaque,
//         Some(ColorSpace::new_srgb()),
//     );
//
//     let mut surface = unsafe {
//         surfaces::wrap_pixels(
//             &image_info,
//             std::slice::from_raw_parts_mut(buffer.as_ptr() as *mut u8, 800 * 500 * BYTE_PER_PIXEL),
//             800 * BYTE_PER_PIXEL,
//             None,
//         )
//     }
//         .unwrap();
//
//     let mut count = 0;
//
//     let mut painter = skia_safe::Paint::default();
//     painter.set_color(<u32 as Into<Color>>::into(0xff0000ffu32));
//
//     while window.is_open() && !window.is_key_down(Key::Escape) {
//         std::thread::sleep(Duration::from_millis(8));
//
//         {
//             let canvas = surface.canvas();
//             canvas.clear(Color4f::from(0xffffffffu32));
//             canvas.draw_rect(Rect::from_ltrb(0.0, 0.0, 500.0, count as f32), &painter);
//         }
//
//         unsafe { window.update_with_buffer(&buffer, 800, 500).unwrap() }
//
//         count += 1;
//         if count > 500 {
//             count = 0;
//         }
//     }
// }

fn main() {
    test(1, 2, Box::new(3));
}
