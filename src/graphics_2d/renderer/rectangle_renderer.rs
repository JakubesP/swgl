use std::marker::PhantomData;
use nalgebra_glm as glm;

use super::super::super::runtime_error::SWGLResult;
use super::super::vertex_2d::interface::{DedicatedShader, VertexType, TextureCoords};
use super::super::super::camera2d::interface::CameraType;
use super::super::super::gl_wrapper::shader::{self, Program};
use super::super::super::gl_wrapper::texture::texture_2d::Texture2D;
use super::super::super::gl_wrapper::vertex_array_object::PrimitiveType;
use super::super::super::gl_wrapper::vertex_array_object::VertexArrayObject;
use super::super::super::gl_wrapper::vertex_buffer_object::VertexBufferObject;
use super::super::super::gl_wrapper::element_buffer_object::ElementBufferObject;
use super::super::super::global_tools::vector2::Vector2;
use super::renderer_conf::RendererConf;
use super::helpers::{create_buffers, create_ebo_buffer};

// -----------------------------------------------------------------------------------------------------------

/// This type is used for renders rectangular geometry for given vertex type. 
pub struct RectangleRenderer<T>
where
    T: VertexType,
{
    phantom: PhantomData<T>,
    program: Program,
    vao: VertexArrayObject,
    vbo: VertexBufferObject,
    ebo: ElementBufferObject,
    vertices: Vec<T>, 
    conf: RendererConf,
}

// -----------------------------------------------------------------------------------------------------------

impl<T> RectangleRenderer<T>
where
    T: VertexType + DedicatedShader,
{
    pub fn init(context: &crate::AppContext, max_rectangle_number: usize) -> SWGLResult<Self> {
        let program = Program::new(
            context,
            &shader::split_vfshader_to_shader_source(T::get_dedicated_shader()),
        )?;

        let (vbo, vao) = create_buffers::<T>(context, max_rectangle_number * 4)?;
        let ebo = create_ebo_buffer(context, max_rectangle_number * 6)?; 

        Ok(Self {
            phantom: PhantomData,
            program,
            vbo,
            vao,
            ebo,
            vertices: Vec::with_capacity(max_rectangle_number * 4), 
            conf: RendererConf::default(),
        })
    }
}

// -----------------------------------------------------------------------------------------------------------

impl<T> RectangleRenderer<T>
where
    T: VertexType,
{
    pub fn init_with_custom_shader(context: &crate::AppContext, max_rectangle_number: usize, program: Program, conf: RendererConf) -> SWGLResult<Self> {
       
        let (vbo, vao) = create_buffers::<T>(context, max_rectangle_number * 4)?;
        let ebo = create_ebo_buffer(context, max_rectangle_number * 6)?; 

        Ok(Self {
            phantom: PhantomData,
            program,
            vbo,
            vao,
            ebo,
            vertices: Vec::with_capacity(max_rectangle_number * 4), 
            conf,
        })
    }
}

// -----------------------------------------------------------------------------------------------------------

impl<T> RectangleRenderer<T>
where
    T: VertexType + Clone,
{
    /// This method prepares given number of textures for shader (for the most scenarios you don't need to use it). 
    pub fn prepare_textures(&self, context: &crate::AppContext, count: usize) -> SWGLResult<()> {
        for i in 0..count {
            self.program.set1i(context, &format!("{}{}", self.conf.texture_uniform_prefix, i), i as i32)?;
        }
        Ok(())
    }

    /// This method activates textures for shader before draw (for the most scenarios you don't need to use it). 
    pub fn bind_textures(&self, context: &crate::AppContext, texture_set: &[&Texture2D]) {
        for (i, tex) in texture_set.iter().enumerate() {
            tex.active(context, i as u32);
        }
    }

    /// This method renders all vertices.
    pub fn flush(&mut self, context: &crate::AppContext, camera: &dyn CameraType, texture: Option<&Texture2D>) -> SWGLResult<()> {
        self.program.use_program(context);

        if let Some(tex) = texture {
            self.program.set1i(context, &format!("{}{}", self.conf.texture_uniform_prefix, 0), 0 as i32)?;
            tex.active(context, 0 as u32);
        }

        self.program.set_mat_4x4f(
            context,
            &self.conf.projection_matrix_uniform_name,
            glm::value_ptr(&camera.matrix()),
        ).unwrap();

        self.program.set_mat_4x4f(
            context,
            &self.conf.model_matrix_uniform_name,
            glm::value_ptr(&camera.dedicated_model()),
        ).unwrap();

        self.vao.bind(context);
        let mut vbo_data = vec![];
        for v in &self.vertices {
            let mut portion = v.to_vec();
            vbo_data.append(&mut portion);
        }

        self.vbo.update_data_safe(context, &vbo_data, 0)?;
        self.vao.draw_elements(context, PrimitiveType::Triangles, 0, (self.vertices.len() / 4 * 6) as u32, &self.ebo);

        self.vertices.clear(); 

        Ok(())
    }

    pub fn program(&self) -> &Program {
        &self.program
    }
}

// -----------------------------------------------------------------------------------------------------------

impl<T> RectangleRenderer<T>
where
    T: VertexType + Clone,
{
    pub fn add_rect(&mut self, vertex: T, position: &Vector2<f32>, size: &Vector2<f32>) {
        let mut tmp = vec![vertex; 4];
        tmp[0].set_position(&Vector2::new(position.x, position.y));
        tmp[1].set_position(&Vector2::new(position.x + size.x, position.y));
        tmp[2].set_position(&Vector2::new(position.x + size.x, position.y + size.y));
        tmp[3].set_position(&Vector2::new(position.x, position.y + size.y));
        self.vertices.append(&mut tmp);
    }

    pub fn add_rect_with_trans(&mut self, vertex: T, position: &Vector2<f32>, size: &Vector2<f32>, origin: &Vector2<f32>, rotation: f32) {
        let mut tmp = vec![vertex; 4];

        tmp[0].set_position(&(Vector2::new(0.0, 0.0) - *origin + *position).rotated_around(rotation, &position));
        tmp[1].set_position(&(Vector2::new(size.x, 0.0) - *origin + *position).rotated_around(rotation, &position));
        tmp[2].set_position(&(Vector2::new(size.x, size.y) - *origin + *position).rotated_around(rotation, &position));
        tmp[3].set_position(&(Vector2::new(0.0, size.y) - *origin + *position).rotated_around(rotation, &position));

        self.vertices.append(&mut tmp);
    }
}

// -----------------------------------------------------------------------------------------------------------

impl<T> RectangleRenderer<T>
where
    T: VertexType + TextureCoords + Clone,
{
    pub fn add_sprite(&mut self, vertex: T, position: &Vector2<f32>, size: &Vector2<f32>) {
        let mut tmp = vec![vertex; 4];

        tmp[0].set_position(&Vector2::new(position.x, position.y));
        tmp[0].set_tex_coords(&Vector2::new(0.0, 0.0));

        tmp[1].set_position(&Vector2::new(position.x + size.x, position.y));
        tmp[1].set_tex_coords(&Vector2::new(1.0, 0.0));

        tmp[2].set_position(&Vector2::new(position.x + size.x, position.y + size.y));
        tmp[2].set_tex_coords(&Vector2::new(1.0, 1.0));

        tmp[3].set_position(&Vector2::new(position.x, position.y + size.y));
        tmp[3].set_tex_coords(&Vector2::new(0.0, 1.0));

        self.vertices.append(&mut tmp);
    }

    pub fn add_sprite_with_trans(&mut self, vertex: T, position: &Vector2<f32>, size: &Vector2<f32>, origin: &Vector2<f32>, rotation: f32) {
        let mut tmp = vec![vertex; 4];

        tmp[0].set_position(&(Vector2::new(0.0, 0.0) - *origin + *position).rotated_around(rotation, &position));
        tmp[0].set_tex_coords(&Vector2::new(0.0, 0.0));

        tmp[1].set_position(&(Vector2::new(size.x, 0.0) - *origin + *position).rotated_around(rotation, &position));
        tmp[1].set_tex_coords(&Vector2::new(1.0, 0.0));

        tmp[2].set_position(&(Vector2::new(size.x, size.y) - *origin + *position).rotated_around(rotation, &position));
        tmp[2].set_tex_coords(&Vector2::new(1.0, 1.0));

        tmp[3].set_position(&(Vector2::new(0.0, size.y) - *origin + *position).rotated_around(rotation, &position));
        tmp[3].set_tex_coords(&Vector2::new(0.0, 1.0));

        self.vertices.append(&mut tmp);
    }
}
