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

    pub const BLACK: Color = Color::new(0xFF, 0x00, 0x00, 0x00);
    pub const DARKGRAY: Color = Color::new(0xFF, 0x44, 0x44, 0x44);
    pub const GRAY: Color = Color::new(0xFF, 0x88, 0x88, 0x88);
    pub const LIGHTGRAY: Color = Color::new(0xFF, 0xCC, 0xCC, 0xCC);
    pub const WHITE: Color = Color::new(0xFF, 0xFF, 0xFF, 0xFF);
    pub const RED: Color = Color::new(0xFF, 0xFF, 0x00, 0x00);
    pub const GREEN: Color = Color::new(0xFF, 0x00, 0xFF, 0x00);
    pub const BLUE: Color = Color::new(0xFF, 0x00, 0x00, 0xFF);
    pub const YELLOW: Color = Color::new(0xFF, 0xFF, 0xFF, 0x00);
    pub const CYAN: Color = Color::new(0xFF, 0x00, 0xFF, 0xFF);
    pub const MAGENTA: Color = Color::new(0xFF, 0xFF, 0x00, 0xFF);
    pub const TRANSPARENT: Color = Color::new(0x00, 0x00, 0x00, 0x00);
}

impl From<Color> for SkiaColor {
    fn from(color: Color) -> Self {
        SkiaColor::from(color.value)
    }
}

impl From<Color> for skia_safe::Color {
    fn from(value: Color) -> Self {
        skia_safe::Color::from(value.value)
    }
}