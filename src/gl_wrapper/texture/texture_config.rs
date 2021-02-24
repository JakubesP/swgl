use super::super::super::global_tools::vector2::Vector2;

// --------------------------------------------------------------------------------------------------

#[derive(Debug, Copy, Clone)]
pub struct TextureCoords {
    pub left_top: Vector2<f32>,
    pub left_bottom: Vector2<f32>,
    pub right_top: Vector2<f32>,
    pub right_bottom: Vector2<f32>,
}

impl TextureCoords {
    pub fn default() -> TextureCoords {
        TextureCoords {
            left_top: Vector2::new(0.0, 0.0),
            left_bottom: Vector2::new(0.0, 1.0),
            right_top: Vector2::new(1.0, 1.0),
            right_bottom: Vector2::new(1.0, 0.0),
        }
    }

    pub fn reverse(&mut self) {
        std::mem::swap(&mut self.left_top.x, &mut self.right_top.x);
        std::mem::swap(&mut self.left_bottom.x, &mut self.right_bottom.x);
    }
}

// --------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub enum TextureFilter {
    Nearest,
    Linear,
    NearestMipmapLinear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    LinearMipmapLinear,
}

impl TextureFilter {
    pub fn get_gl_property_value(&self) -> u32 {
        match self {
            Self::Nearest => crate::AppContext::NEAREST,
            Self::Linear => crate::AppContext::LINEAR,
            Self::NearestMipmapLinear => crate::AppContext::NEAREST_MIPMAP_LINEAR,
            Self::NearestMipmapNearest => crate::AppContext::NEAREST_MIPMAP_NEAREST,
            Self::LinearMipmapNearest => crate::AppContext::LINEAR_MIPMAP_NEAREST,
            Self::LinearMipmapLinear => crate::AppContext::LINEAR_MIPMAP_LINEAR,
        }
    }
}

// --------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub enum TextureWrap {
    Repeat,
    MirroredRepeat,
    ClampToEdge,
}

impl TextureWrap {
    pub fn get_gl_property_value(&self) -> u32 {
        match self {
            Self::Repeat => crate::AppContext::REPEAT,
            Self::MirroredRepeat => crate::AppContext::MIRRORED_REPEAT,
            Self::ClampToEdge => crate::AppContext::CLAMP_TO_EDGE,
        }
    }
}

// --------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub enum TextureChannels {
    Rgb,
    Rgba,
}

impl TextureChannels {
    pub fn get_gl_property_value(&self) -> u32 {
        match self {
            Self::Rgb => crate::AppContext::RGB,
            Self::Rgba => crate::AppContext::RGBA,
        }
    }
}

// --------------------------------------------------------------------------------------------------

pub struct TextureConfiguration {
    pub wrap_x: TextureWrap,
    pub wrap_y: TextureWrap,
    pub min_filter: TextureFilter,
    pub mag_filter: TextureFilter,
    pub chanels: TextureChannels,
}

impl TextureConfiguration {
    pub fn new(
        wrap_x: TextureWrap,
        wrap_y: TextureWrap,
        min_filter: TextureFilter,
        mag_filter: TextureFilter,
        chanels: TextureChannels,
    ) -> TextureConfiguration {
        TextureConfiguration {
            wrap_x,
            wrap_y,
            min_filter,
            mag_filter,
            chanels,
        }
    }

    pub fn default() -> TextureConfiguration {
        TextureConfiguration {
            wrap_x: TextureWrap::ClampToEdge,
            wrap_y: TextureWrap::ClampToEdge,
            min_filter: TextureFilter::LinearMipmapLinear,
            mag_filter: TextureFilter::LinearMipmapLinear,
            chanels: TextureChannels::Rgba,
        }
    }
}