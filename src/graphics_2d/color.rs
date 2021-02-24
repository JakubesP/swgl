
pub mod mask {
    pub const RED:   u32 = 0xff000000;
    pub const GREEN: u32 = 0x00ff0000;
    pub const BLUE:  u32 = 0x0000ff00;
    pub const ALPHA: u32 = 0x000000ff;
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Color { red, green, blue, alpha }
    }

    pub fn from_bytes(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Color::new(red as f32 / 255.0, green as f32 / 255.0, blue as f32 / 255.0, alpha as f32 / 255.0)
    }

    pub fn from_hex(color: u32) -> Self {
        let r = (color & mask::RED)   >> 24;
        let g = (color & mask::GREEN) >> 16;
        let b = (color & mask::BLUE)  >>  8;
        let a =  color & mask::ALPHA;
        Color::from_bytes(r as u8, g as u8, b as u8, a as u8)
    }
}
