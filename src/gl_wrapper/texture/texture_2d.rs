use web_sys::{WebGlTexture, HtmlImageElement};
use super::texture_config::TextureConfiguration;
use super::super::super::runtime_error::{SWGLResult, SWGLRuntimeError};
use super::super::super::global_tools::vector2::Vector2;

// --------------------------------------------------------------------------------------------------

pub struct Texture2D {
    texture: std::option::Option<WebGlTexture>,
    texture_dim: Vector2<u32>,
    configuration: TextureConfiguration,
    texture_type: u32,
}

// --------------------------------------------------------------------------------------------------

impl Texture2D {

    pub fn new_texture2d(context: &crate::AppContext, img: &HtmlImageElement, conf: TextureConfiguration) -> SWGLResult<Self> {
        let buffer = context.create_texture();
        let texture = Self {
            texture: buffer,
            texture_dim: Vector2::new(img.width(), img.height()),
            configuration: conf,
            texture_type: crate::AppContext::TEXTURE_2D,
        };

        texture.bind(context);
        texture.config_texture_2d(context, img)?;
        texture.general_config(context);


        Ok(texture)
    }

    pub fn new_texture_array(context: &crate::AppContext, img: &HtmlImageElement, conf: TextureConfiguration, slice_dim: Vector2<f32>, slice_count: u32) -> SWGLResult<Self> {
        let buffer = context.create_texture();

        let texture = Texture2D {
            texture: buffer,
            texture_dim: Vector2::new(img.width(), img.height()),
            configuration: conf,
            texture_type: crate::AppContext::TEXTURE_2D_ARRAY,
        };

        texture.bind(context);
        texture.config_texture_array(context, img, slice_dim, slice_count)?;
        texture.general_config(context);

        Ok(texture)
    }

    fn general_config(&self, context: &crate::AppContext) {
        context.tex_parameteri(
            self.texture_type,
            crate::AppContext::TEXTURE_WRAP_S,
            self.configuration.wrap_x.get_gl_property_value() as i32,
        );
        context.tex_parameteri(
            self.texture_type,
            crate::AppContext::TEXTURE_WRAP_T,
            self.configuration.wrap_y.get_gl_property_value() as i32,
        );

        context.tex_parameteri(
            self.texture_type,
            crate::AppContext::TEXTURE_MIN_FILTER,
            self.configuration.min_filter.get_gl_property_value() as i32,
        );

        context.tex_parameteri(
            self.texture_type,
            crate::AppContext::TEXTURE_MAG_FILTER,
            self.configuration.mag_filter.get_gl_property_value() as i32,
        );
    } 

    // 2d texture
    fn config_texture_2d(&self, context: &crate::AppContext, img: &HtmlImageElement) -> SWGLResult<()> {

        context.tex_image_2d_with_u32_and_u32_and_html_image_element(
            crate::AppContext::TEXTURE_2D,
            0,
            self.configuration.chanels.get_gl_property_value() as i32,
            self.configuration.chanels.get_gl_property_value(),
            crate::AppContext::UNSIGNED_BYTE,
            img,
        ).ok().ok_or(SWGLRuntimeError::new("Texture2D::CannotConfigTexture"))?;

        context.generate_mipmap(crate::AppContext::TEXTURE_2D);

        Ok(())
    }

    // texture array
    fn config_texture_array(&self, context: &crate::AppContext, img: &HtmlImageElement, slice_dim: Vector2<f32>, slice_count: u32) -> SWGLResult<()> {

        context.tex_image_3d_with_html_image_element(
            crate::AppContext::TEXTURE_2D_ARRAY,
            0,                                                         // mipmap level
            self.configuration.chanels.get_gl_property_value() as i32, // gpu texel format
            slice_dim.x as i32,                                        // texture file pixel width 
            slice_dim.y as i32,                                        // height file pixel height
            slice_count as i32,                                        // depth ( how many slices )
            0,                                                         // border
            self.configuration.chanels.get_gl_property_value(),        // cpu pixel format
            crate::AppContext::UNSIGNED_BYTE,                                         // cpu pixel coord type
            img,                                                       // pixel data
        ).ok().ok_or(SWGLRuntimeError::new("Texture2D::CannotConfigTextureArray"))?;

        context.generate_mipmap(crate::AppContext::TEXTURE_2D_ARRAY);

        Ok(())
    }

    pub fn bind(&self, context: &crate::AppContext) {
        context.bind_texture(self.texture_type, self.texture.as_ref());
    }

    pub fn active(&self, context: &crate::AppContext, location: u32) {
        context.active_texture(crate::AppContext::TEXTURE0 + location);
        self.bind(context);
    }

    // -----------------------------------

    /// This method returns object that can be used directly inside WebGL low level calls.
    pub fn get_raw_id(&self) -> &std::option::Option<WebGlTexture> {
        &self.texture
    }

    pub fn get_size(&self) -> (f32, f32) {
        (self.texture_dim.x as f32, self.texture_dim.y as f32)
    }
}



