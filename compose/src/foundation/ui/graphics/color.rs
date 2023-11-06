use skia_safe::Color4f as SkiaColor;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    value: u32,
}

impl Color {
    pub const fn new(alpha: u8, red: u8, green: u8, blue: u8) -> Color {
        Color { value: (alpha as u32) << 24 | (red as u32) << 16 | (green as u32) << 8 | (blue as u32) }
    }

    pub fn alpha(&self) -> f32 {
        (self.value >> 24) as f32 / 255.0
    }

    pub fn red(&self) -> f32 {
        ((self.value >> 16) & 0xFF) as f32 / 255.0
    }

    pub fn green(&self) -> f32 {
        ((self.value >> 8) & 0xFF) as f32 / 255.0
    }

    pub fn blue(&self) -> f32 {
        (self.value & 0xFF) as f32 / 255.0
    }

    const BLACK: Color = Color::new(0xFF, 0x00, 0x00, 0x00);
    const DARKGRAY: Color = Color::new(0xFF, 0x44, 0x44, 0x44);
    const GRAY: Color = Color::new(0xFF, 0x88, 0x88, 0x88);
    const LIGHTGRAY: Color = Color::new(0xFF, 0xCC, 0xCC, 0xCC);
    const WHITE: Color = Color::new(0xFF, 0xFF, 0xFF, 0xFF);
    const RED: Color = Color::new(0xFF, 0xFF, 0x00, 0x00);
    const GREEN: Color = Color::new(0xFF, 0x00, 0xFF, 0x00);
    const BLUE: Color = Color::new(0xFF, 0x00, 0x00, 0xFF);
    const YELLOW: Color = Color::new(0xFF, 0xFF, 0xFF, 0x00);
    const CYAN: Color = Color::new(0xFF, 0x00, 0xFF, 0xFF);
    const MAGENTA: Color = Color::new(0xFF, 0xFF, 0x00, 0xFF);
    const TRANSPARENT: Color = Color::new(0x00, 0x00, 0x00, 0x00);
}

impl From<Color> for SkiaColor {
    fn from(color: Color) -> Self {
        SkiaColor::from(color.value)
    }
}